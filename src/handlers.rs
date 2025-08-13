use crate::{
    auth::{extract_auth_user, hash_password, verify_password, create_jwt},
    db::*,
    error::AppResult,
    models::*,
};
use actix_web::{web, HttpRequest, HttpResponse};
use time::{Date, OffsetDateTime};
use uuid::Uuid;
use validator::Validate;

// Auth handlers
pub async fn register(
    pool: web::Data<sqlx::PgPool>,
    user_data: web::Json<CreateUserRequest>,
) -> AppResult<HttpResponse> {
    user_data.validate()?;

    // Check if user already exists
    let existing_user = get_user_by_email(&pool, &user_data.email).await?;
    if existing_user.is_some() {
        return Err(crate::error::AppError::BadRequest(
            "User with this email already exists".to_string(),
        ));
    }

    // Hash password
    let password_hash = hash_password(&user_data.password).await?;

    // Create user with default role as User
    let user = create_user(
        &pool,
        &user_data.email,
        &password_hash,
        &user_data.first_name,
        &user_data.last_name,
        UserRole::User,
    )
    .await?;

    let response = UserResponse {
        id: user.id,
        email: user.email,
        first_name: user.first_name,
        last_name: user.last_name,
        role: user.role,
    };

    Ok(HttpResponse::Created().json(response))
}

pub async fn login(
    pool: web::Data<sqlx::PgPool>,
    login_data: web::Json<LoginRequest>,
) -> AppResult<HttpResponse> {
    login_data.validate()?;

    // Find user by email
    let user = get_user_by_email(&pool, &login_data.email)
        .await?
        .ok_or_else(|| {
            crate::error::AppError::Authentication("Invalid email or password".to_string())
        })?;

    // Verify password
    let is_valid = verify_password(&login_data.password, &user.password_hash).await?;
    if !is_valid {
        return Err(crate::error::AppError::Authentication(
            "Invalid email or password".to_string(),
        ));
    }

    // Check if user is active
    if !user.is_active {
        return Err(crate::error::AppError::Authentication(
            "Account is deactivated".to_string(),
        ));
    }

    // Create JWT token
    let token = create_jwt(user.id, user.role.clone())?;

    let response = LoginResponse {
        token,
        user: UserResponse {
            id: user.id,
            email: user.email,
            first_name: user.first_name,
            last_name: user.last_name,
            role: user.role,
        },
    };

    Ok(HttpResponse::Ok().json(response))
}

// Property handlers
pub async fn get_properties(
    pool: web::Data<sqlx::PgPool>,
    query: web::Query<PropertyFilters>,
) -> AppResult<HttpResponse> {
    let filters = query.into_inner();
    
    let (properties, total) = get_properties_with_filters(&pool, &filters).await?;
    
    let page = filters.page.unwrap_or(1);
    let per_page = filters.per_page.unwrap_or(10);
    let total_pages = (total + per_page - 1) / per_page;

    // Convert to PropertyResponse with owner info
    let mut property_responses = Vec::new();
    for property in properties {
        let owner = get_user_by_id(&pool, property.owner_id).await?
            .ok_or_else(|| crate::error::AppError::NotFound("Owner not found".to_string()))?;
        
        let owner_response = UserResponse {
            id: owner.id,
            email: owner.email,
            first_name: owner.first_name,
            last_name: owner.last_name,
            role: owner.role,
        };

        let property_response = PropertyResponse {
            id: property.id,
            title: property.title,
            description: property.description,
            property_type: property.property_type,
            location: property.location,
            address: property.address,
            city: property.city,
            country: property.country,
            price_per_night: property.price_per_night,
            max_guests: property.max_guests,
            bedrooms: property.bedrooms,
            bathrooms: property.bathrooms,
            amenities: property.amenities,
            images: property.images,
            owner: owner_response,
        };

        property_responses.push(property_response);
    }

    let response = PaginatedResponse {
        data: property_responses,
        total,
        page,
        per_page,
        total_pages,
    };

    Ok(HttpResponse::Ok().json(response))
}

pub async fn get_property(
    pool: web::Data<sqlx::PgPool>,
    path: web::Path<Uuid>,
) -> AppResult<HttpResponse> {
    let property_id = path.into_inner();
    
    let property = get_property_by_id(&pool, property_id)
        .await?
        .ok_or_else(|| crate::error::AppError::NotFound("Property not found".to_string()))?;

    let owner = get_user_by_id(&pool, property.owner_id).await?
        .ok_or_else(|| crate::error::AppError::NotFound("Owner not found".to_string()))?;
    
    let owner_response = UserResponse {
        id: owner.id,
        email: owner.email,
        first_name: owner.first_name,
        last_name: owner.last_name,
        role: owner.role,
    };

    let response = PropertyResponse {
        id: property.id,
        title: property.title,
        description: property.description,
        property_type: property.property_type,
        location: property.location,
        address: property.address,
        city: property.city,
        country: property.country,
        price_per_night: property.price_per_night,
        max_guests: property.max_guests,
        bedrooms: property.bedrooms,
        bathrooms: property.bathrooms,
        amenities: property.amenities,
        images: property.images,
        owner: owner_response,
    };

    Ok(HttpResponse::Ok().json(response))
}

pub async fn create_property(
    pool: web::Data<sqlx::PgPool>,
    req: HttpRequest,
    property_data: web::Json<CreatePropertyRequest>,
) -> AppResult<HttpResponse> {
    property_data.validate()?;
    
    let auth_user = extract_auth_user(&req)?;
    
    // Only property owners and admins can create properties
    if auth_user.role != UserRole::PropertyOwner && auth_user.role != UserRole::Admin {
        return Err(crate::error::AppError::Authorization(
            "Only property owners can create properties".to_string(),
        ));
    }

    let property = create_property_db(&pool, auth_user.id, &property_data).await?;

    Ok(HttpResponse::Created().json(property))
}

pub async fn get_my_properties(
    pool: web::Data<sqlx::PgPool>,
    req: HttpRequest,
) -> AppResult<HttpResponse> {
    let auth_user = extract_auth_user(&req)?;
    
    // Only property owners and admins can view their properties
    if auth_user.role != UserRole::PropertyOwner && auth_user.role != UserRole::Admin {
        return Err(crate::error::AppError::Authorization(
            "Only property owners can view their properties".to_string(),
        ));
    }

    let properties = get_properties_by_owner(&pool, auth_user.id).await?;

    Ok(HttpResponse::Ok().json(properties))
}

// Booking handlers
pub async fn create_booking(
    pool: web::Data<sqlx::PgPool>,
    req: HttpRequest,
    booking_data: web::Json<CreateBookingRequest>,
) -> AppResult<HttpResponse> {
    booking_data.validate()?;
    
    let auth_user = extract_auth_user(&req)?;
    
    // Only regular users can create bookings
    if auth_user.role != UserRole::User {
        return Err(crate::error::AppError::Authorization(
            "Only users can create bookings".to_string(),
        ));
    }

    let property_id = booking_data.property_id;
    
    // Check if property exists
    let _property = get_property_by_id(&pool, property_id)
        .await?
        .ok_or_else(|| crate::error::AppError::NotFound("Property not found".to_string()))?;

    // For now, we'll use a simple date range (next day to next day + 1)
    // In a real app, you'd get these from the request
    let check_in_date = Date::from_ordinal_date(
        OffsetDateTime::now_utc().year(),
        OffsetDateTime::now_utc().ordinal() + 1
    ).unwrap_or_else(|_| Date::from_ordinal_date(2024, 1).unwrap());
    
    let check_out_date = Date::from_ordinal_date(
        OffsetDateTime::now_utc().year(),
        OffsetDateTime::now_utc().ordinal() + 2
    ).unwrap_or_else(|_| Date::from_ordinal_date(2024, 2).unwrap());

    // Check availability
    let is_available = check_property_availability(&pool, property_id, check_in_date, check_out_date).await?;
    if !is_available {
        return Err(crate::error::AppError::BadRequest(
            "Property is not available for the selected dates".to_string(),
        ));
    }

    // Calculate total price
    let total_price = calculate_booking_price(&pool, property_id, check_in_date, check_out_date, booking_data.guest_count).await?;

    let booking = create_booking_db(
        &pool,
        property_id,
        auth_user.id,
        check_in_date,
        check_out_date,
        total_price,
        booking_data.guest_count,
        booking_data.special_requests.as_deref(),
    )
    .await?;

    Ok(HttpResponse::Created().json(booking))
}

pub async fn get_my_bookings(
    pool: web::Data<sqlx::PgPool>,
    req: HttpRequest,
) -> AppResult<HttpResponse> {
    let auth_user = extract_auth_user(&req)?;
    
    let bookings = get_bookings_by_user(&pool, auth_user.id).await?;

    // Convert to BookingResponse with property and user info
    let mut booking_responses = Vec::new();
    for booking in bookings {
        let property = get_property_by_id(&pool, booking.property_id).await?
            .ok_or_else(|| crate::error::AppError::NotFound("Property not found".to_string()))?;
        
        let owner = get_user_by_id(&pool, property.owner_id).await?
            .ok_or_else(|| crate::error::AppError::NotFound("Owner not found".to_string()))?;
        
        let owner_response = UserResponse {
            id: owner.id,
            email: owner.email,
            first_name: owner.first_name,
            last_name: owner.last_name,
            role: owner.role,
        };

        let property_response = PropertyResponse {
            id: property.id,
            title: property.title,
            description: property.description,
            property_type: property.property_type,
            location: property.location,
            address: property.address,
            city: property.city,
            country: property.country,
            price_per_night: property.price_per_night,
            max_guests: property.max_guests,
            bedrooms: property.bedrooms,
            bathrooms: property.bathrooms,
            amenities: property.amenities,
            images: property.images,
            owner: owner_response,
        };

        let user_response = UserResponse {
            id: auth_user.id,
            email: auth_user.email.clone(),
            first_name: "".to_string(), // You'd get this from the user record
            last_name: "".to_string(),
            role: auth_user.role.clone(),
        };

        let booking_response = BookingResponse {
            id: booking.id,
            property: property_response,
            user: user_response,
            check_in_date: booking.check_in_date,
            check_out_date: booking.check_out_date,
            total_price: booking.total_price,
            status: booking.status,
            guest_count: booking.guest_count,
            special_requests: booking.special_requests,
            created_at: booking.created_at,
        };

        booking_responses.push(booking_response);
    }

    Ok(HttpResponse::Ok().json(booking_responses))
}

pub async fn cancel_booking(
    pool: web::Data<sqlx::PgPool>,
    req: HttpRequest,
    path: web::Path<Uuid>,
) -> AppResult<HttpResponse> {
    let booking_id = path.into_inner();
    let auth_user = extract_auth_user(&req)?;
    
    let booking = get_booking_by_id(&pool, booking_id)
        .await?
        .ok_or_else(|| crate::error::AppError::NotFound("Booking not found".to_string()))?;

    // Users can only cancel their own bookings
    if auth_user.role == UserRole::User && booking.user_id != auth_user.id {
        return Err(crate::error::AppError::Authorization(
            "You can only cancel your own bookings".to_string(),
        ));
    }

    // Property owners can cancel bookings for their properties
    if auth_user.role == UserRole::PropertyOwner {
        let property = get_property_by_id(&pool, booking.property_id).await?
            .ok_or_else(|| crate::error::AppError::NotFound("Property not found".to_string()))?;
        
        if property.owner_id != auth_user.id {
            return Err(crate::error::AppError::Authorization(
                "You can only cancel bookings for your own properties".to_string(),
            ));
        }
    }

    // Only pending or approved bookings can be cancelled
    if booking.status != BookingStatus::Pending && booking.status != BookingStatus::Approved {
        return Err(crate::error::AppError::BadRequest(
            "Only pending or approved bookings can be cancelled".to_string(),
        ));
    }

    let updated_booking = update_booking_status(&pool, booking_id, BookingStatus::Cancelled).await?;

    Ok(HttpResponse::Ok().json(updated_booking))
}

pub async fn approve_booking(
    pool: web::Data<sqlx::PgPool>,
    req: HttpRequest,
    path: web::Path<Uuid>,
) -> AppResult<HttpResponse> {
    let booking_id = path.into_inner();
    let auth_user = extract_auth_user(&req)?;
    
    // Only property owners and admins can approve bookings
    if auth_user.role != UserRole::PropertyOwner && auth_user.role != UserRole::Admin {
        return Err(crate::error::AppError::Authorization(
            "Only property owners can approve bookings".to_string(),
        ));
    }

    let booking = get_booking_by_id(&pool, booking_id)
        .await?
        .ok_or_else(|| crate::error::AppError::NotFound("Booking not found".to_string()))?;

    // Property owners can only approve bookings for their properties
    if auth_user.role == UserRole::PropertyOwner {
        let property = get_property_by_id(&pool, booking.property_id).await?
            .ok_or_else(|| crate::error::AppError::NotFound("Property not found".to_string()))?;
        
        if property.owner_id != auth_user.id {
            return Err(crate::error::AppError::Authorization(
                "You can only approve bookings for your own properties".to_string(),
            ));
        }
    }

    // Only pending bookings can be approved
    if booking.status != BookingStatus::Pending {
        return Err(crate::error::AppError::BadRequest(
            "Only pending bookings can be approved".to_string(),
        ));
    }

    let updated_booking = update_booking_status(&pool, booking_id, BookingStatus::Approved).await?;

    Ok(HttpResponse::Ok().json(updated_booking))
}

pub async fn deny_booking(
    pool: web::Data<sqlx::PgPool>,
    req: HttpRequest,
    path: web::Path<Uuid>,
) -> AppResult<HttpResponse> {
    let booking_id = path.into_inner();
    let auth_user = extract_auth_user(&req)?;
    
    // Only property owners and admins can deny bookings
    if auth_user.role != UserRole::PropertyOwner && auth_user.role != UserRole::Admin {
        return Err(crate::error::AppError::Authorization(
            "Only property owners can deny bookings".to_string(),
        ));
    }

    let booking = get_booking_by_id(&pool, booking_id)
        .await?
        .ok_or_else(|| crate::error::AppError::NotFound("Booking not found".to_string()))?;

    // Property owners can only deny bookings for their properties
    if auth_user.role == UserRole::PropertyOwner {
        let property = get_property_by_id(&pool, booking.property_id).await?
            .ok_or_else(|| crate::error::AppError::NotFound("Property not found".to_string()))?;
        
        if property.owner_id != auth_user.id {
            return Err(crate::error::AppError::Authorization(
                "You can only deny bookings for your own properties".to_string(),
            ));
        }
    }

    // Only pending bookings can be denied
    if booking.status != BookingStatus::Pending {
        return Err(crate::error::AppError::BadRequest(
            "Only pending bookings can be denied".to_string(),
        ));
    }

    let updated_booking = update_booking_status(&pool, booking_id, BookingStatus::Denied).await?;

    Ok(HttpResponse::Ok().json(updated_booking))
}

// Admin handlers
pub async fn create_property_owner(
    pool: web::Data<sqlx::PgPool>,
    req: HttpRequest,
    user_data: web::Json<CreateUserRequest>,
) -> AppResult<HttpResponse> {
    user_data.validate()?;
    
    let auth_user = extract_auth_user(&req)?;
    
    // Only admins can create property owners
    if auth_user.role != UserRole::Admin {
        return Err(crate::error::AppError::Authorization(
            "Only admins can create property owners".to_string(),
        ));
    }

    // Check if user already exists
    let existing_user = get_user_by_email(&pool, &user_data.email).await?;
    if existing_user.is_some() {
        return Err(crate::error::AppError::BadRequest(
            "User with this email already exists".to_string(),
        ));
    }

    // Hash password
    let password_hash = hash_password(&user_data.password).await?;

    // Create user with PropertyOwner role
    let user = create_user(
        &pool,
        &user_data.email,
        &password_hash,
        &user_data.first_name,
        &user_data.last_name,
        UserRole::PropertyOwner,
    )
    .await?;

    let response = UserResponse {
        id: user.id,
        email: user.email,
        first_name: user.first_name,
        last_name: user.last_name,
        role: user.role,
    };

    Ok(HttpResponse::Created().json(response))
}

pub async fn get_property_bookings(
    pool: web::Data<sqlx::PgPool>,
    req: HttpRequest,
    path: web::Path<Uuid>,
) -> AppResult<HttpResponse> {
    let property_id = path.into_inner();
    let auth_user = extract_auth_user(&req)?;
    
    // Only property owners and admins can view property bookings
    if auth_user.role != UserRole::PropertyOwner && auth_user.role != UserRole::Admin {
        return Err(crate::error::AppError::Authorization(
            "Only property owners can view property bookings".to_string(),
        ));
    }

    // Property owners can only view bookings for their properties
    if auth_user.role == UserRole::PropertyOwner {
        let property = get_property_by_id(&pool, property_id).await?
            .ok_or_else(|| crate::error::AppError::NotFound("Property not found".to_string()))?;
        
        if property.owner_id != auth_user.id {
            return Err(crate::error::AppError::Authorization(
                "You can only view bookings for your own properties".to_string(),
            ));
        }
    }

    let bookings = get_bookings_by_property(&pool, property_id).await?;

    Ok(HttpResponse::Ok().json(bookings))
}
