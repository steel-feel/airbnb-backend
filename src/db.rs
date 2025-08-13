use crate::{error::AppError, models::*};
use sqlx::{PgPool, Row};
use time::Date;
use uuid::Uuid;

pub async fn create_pool(database_url: &str) -> Result<PgPool, AppError> {
    let pool = PgPool::connect(database_url).await?;
    Ok(pool)
}

// Helper function to convert database row to User
pub fn row_to_user(row: sqlx::postgres::PgRow) -> Result<User, AppError> {
    Ok(User {
        id: row.try_get("id")?,
        email: row.try_get("email")?,
        password_hash: row.try_get("password_hash")?,
        first_name: row.try_get("first_name")?,
        last_name: row.try_get("last_name")?,
        role: match row.try_get::<&str, _>("role")? {
            "user" => UserRole::User,
            "property_owner" => UserRole::PropertyOwner,
            "admin" => UserRole::Admin,
            _ => return Err(AppError::Internal("Invalid user role".to_string())),
        },
        is_active: row.try_get("is_active")?,
        created_at: row.try_get("created_at")?,
        updated_at: row.try_get("updated_at")?,
    })
}

// Helper function to convert database row to Property
pub fn row_to_property(row: sqlx::postgres::PgRow) -> Result<Property, AppError> {
    Ok(Property {
        id: row.try_get("id")?,
        owner_id: row.try_get("owner_id")?,
        title: row.try_get("title")?,
        description: row.try_get("description")?,
        property_type: match row.try_get::<&str, _>("property_type")? {
            "hotel" => PropertyType::Hotel,
            "hostel" => PropertyType::Hostel,
            "apartment" => PropertyType::Apartment,
            _ => return Err(AppError::Internal("Invalid property type".to_string())),
        },
        location: row.try_get("location")?,
        address: row.try_get("address")?,
        city: row.try_get("city")?,
        country: row.try_get("country")?,
        postal_code: row.try_get("postal_code")?,
        latitude: row.try_get("latitude")?,
        longitude: row.try_get("longitude")?,
        price_per_night: row.try_get("price_per_night")?,
        max_guests: row.try_get("max_guests")?,
        bedrooms: row.try_get("bedrooms")?,
        bathrooms: row.try_get("bathrooms")?,
        amenities: row.try_get("amenities")?,
        images: row.try_get("images")?,
        is_active: row.try_get("is_active")?,
        created_at: row.try_get("created_at")?,
        updated_at: row.try_get("updated_at")?,
    })
}

// Helper function to convert database row to Booking
pub fn row_to_booking(row: sqlx::postgres::PgRow) -> Result<Booking, AppError> {
    Ok(Booking {
        id: row.try_get("id")?,
        property_id: row.try_get("property_id")?,
        user_id: row.try_get("user_id")?,
        check_in_date: row.try_get("check_in_date")?,
        check_out_date: row.try_get("check_out_date")?,
        total_price: row.try_get("total_price")?,
        status: match row.try_get::<&str, _>("status")? {
            "pending" => BookingStatus::Pending,
            "approved" => BookingStatus::Approved,
            "denied" => BookingStatus::Denied,
            "cancelled" => BookingStatus::Cancelled,
            "completed" => BookingStatus::Completed,
            _ => return Err(AppError::Internal("Invalid booking status".to_string())),
        },
        guest_count: row.try_get("guest_count")?,
        special_requests: row.try_get("special_requests")?,
        created_at: row.try_get("created_at")?,
        updated_at: row.try_get("updated_at")?,
    })
}

// User operations
pub async fn create_user(
    pool: &PgPool,
    email: &str,
    password_hash: &str,
    first_name: &str,
    last_name: &str,
    role: UserRole,
) -> Result<User, AppError> {
    let role_str = match role {
        UserRole::User => "user",
        UserRole::PropertyOwner => "property_owner",
        UserRole::Admin => "admin",
    };

    let row = sqlx::query(
        r#"
        INSERT INTO users (email, password_hash, first_name, last_name, role)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING *
        "#
    )
    .bind(email)
    .bind(password_hash)
    .bind(first_name)
    .bind(last_name)
    .bind(role_str)
    .fetch_one(pool)
    .await?;

    row_to_user(row)
}

pub async fn get_user_by_email(pool: &PgPool, email: &str) -> Result<Option<User>, AppError> {
    let row = sqlx::query("SELECT * FROM users WHERE email = $1")
        .bind(email)
        .fetch_optional(pool)
        .await?;

    match row {
        Some(row) => Ok(Some(row_to_user(row)?)),
        None => Ok(None),
    }
}

pub async fn get_user_by_id(pool: &PgPool, id: Uuid) -> Result<Option<User>, AppError> {
    let row = sqlx::query("SELECT * FROM users WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await?;

    match row {
        Some(row) => Ok(Some(row_to_user(row)?)),
        None => Ok(None),
    }
}

// Property operations
pub async fn create_property_db(
    pool: &PgPool,
    owner_id: Uuid,
    request: &CreatePropertyRequest,
) -> Result<Property, AppError> {
    let property_type_str = match request.property_type {
        PropertyType::Hotel => "hotel",
        PropertyType::Hostel => "hostel",
        PropertyType::Apartment => "apartment",
    };

    let row = sqlx::query(
        r#"
        INSERT INTO properties (
            owner_id, title, description, property_type, location, address, city, country,
            postal_code, latitude, longitude, price_per_night, max_guests, bedrooms,
            bathrooms, amenities, images
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17)
        RETURNING *
        "#
    )
    .bind(owner_id)
    .bind(&request.title)
    .bind(&request.description)
    .bind(property_type_str)
    .bind(&request.location)
    .bind(&request.address)
    .bind(&request.city)
    .bind(&request.country)
    .bind(&request.postal_code)
    .bind(request.latitude)
    .bind(request.longitude)
    .bind(request.price_per_night)
    .bind(request.max_guests)
    .bind(request.bedrooms)
    .bind(request.bathrooms)
    .bind(&request.amenities)
    .bind(&request.images)
    .fetch_one(pool)
    .await?;

    row_to_property(row)
}

pub async fn get_properties_with_filters(
    pool: &PgPool,
    filters: &PropertyFilters,
) -> Result<(Vec<Property>, i64), AppError> {
    // For now, return a simple query without complex filtering
    // In production, you'd implement proper dynamic query building
    let rows = sqlx::query(
        "SELECT * FROM properties WHERE is_active = true ORDER BY created_at DESC LIMIT $1 OFFSET $2"
    )
    .bind(filters.per_page.unwrap_or(10))
    .bind((filters.page.unwrap_or(1) - 1) * filters.per_page.unwrap_or(10))
    .fetch_all(pool)
    .await?;

    let properties: Result<Vec<Property>, AppError> = rows.into_iter()
        .map(row_to_property)
        .collect();
    let properties = properties?;

    let total = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM properties WHERE is_active = true"
    )
    .fetch_one(pool)
    .await?;

    Ok((properties, total))
}

pub async fn get_property_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Property>, AppError> {
    let row = sqlx::query("SELECT * FROM properties WHERE id = $1 AND is_active = true")
        .bind(id)
        .fetch_optional(pool)
        .await?;

    match row {
        Some(row) => Ok(Some(row_to_property(row)?)),
        None => Ok(None),
    }
}

pub async fn get_properties_by_owner(
    pool: &PgPool,
    owner_id: Uuid,
) -> Result<Vec<Property>, AppError> {
    let rows = sqlx::query("SELECT * FROM properties WHERE owner_id = $1 ORDER BY created_at DESC")
        .bind(owner_id)
        .fetch_all(pool)
        .await?;

    let properties: Result<Vec<Property>, AppError> = rows.into_iter()
        .map(row_to_property)
        .collect();

    properties
}

// Booking operations
pub async fn create_booking_db(
    pool: &PgPool,
    property_id: Uuid,
    user_id: Uuid,
    check_in_date: Date,
    check_out_date: Date,
    total_price: i32,
    guest_count: i32,
    special_requests: Option<&str>,
) -> Result<Booking, AppError> {
    let row = sqlx::query(
        r#"
        INSERT INTO bookings (
            property_id, user_id, check_in_date, check_out_date, total_price, guest_count, special_requests
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING *
        "#
    )
    .bind(property_id)
    .bind(user_id)
    .bind(check_in_date)
    .bind(check_out_date)
    .bind(total_price)
    .bind(guest_count)
    .bind(special_requests)
    .fetch_one(pool)
    .await?;

    row_to_booking(row)
}

pub async fn get_booking_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Booking>, AppError> {
    let row = sqlx::query("SELECT * FROM bookings WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await?;

    match row {
        Some(row) => Ok(Some(row_to_booking(row)?)),
        None => Ok(None),
    }
}

pub async fn get_bookings_by_user(pool: &PgPool, user_id: Uuid) -> Result<Vec<Booking>, AppError> {
    let rows = sqlx::query("SELECT * FROM bookings WHERE user_id = $1 ORDER BY created_at DESC")
        .bind(user_id)
        .fetch_all(pool)
        .await?;

    let bookings: Result<Vec<Booking>, AppError> = rows.into_iter()
        .map(row_to_booking)
        .collect();

    bookings
}

pub async fn get_bookings_by_property(pool: &PgPool, property_id: Uuid) -> Result<Vec<Booking>, AppError> {
    let rows = sqlx::query("SELECT * FROM bookings WHERE property_id = $1 ORDER BY created_at DESC")
        .bind(property_id)
        .fetch_all(pool)
        .await?;

    let bookings: Result<Vec<Booking>, AppError> = rows.into_iter()
        .map(row_to_booking)
        .collect();

    bookings
}

pub async fn update_booking_status(
    pool: &PgPool,
    booking_id: Uuid,
    status: BookingStatus,
) -> Result<Booking, AppError> {
    let status_str = match status {
        BookingStatus::Pending => "pending",
        BookingStatus::Approved => "approved",
        BookingStatus::Denied => "denied",
        BookingStatus::Cancelled => "cancelled",
        BookingStatus::Completed => "completed",
    };

    let row = sqlx::query(
        r#"
        UPDATE bookings 
        SET status = $1, updated_at = NOW()
        WHERE id = $2
        RETURNING *
        "#
    )
    .bind(status_str)
    .bind(booking_id)
    .fetch_one(pool)
    .await?;

    row_to_booking(row)
}

// Check property availability
pub async fn check_property_availability(
    pool: &PgPool,
    property_id: Uuid,
    check_in_date: Date,
    check_out_date: Date,
) -> Result<bool, AppError> {
    let row = sqlx::query(
        r#"
        SELECT COUNT(*) as count
        FROM bookings
        WHERE property_id = $1 
        AND status IN ('pending', 'approved')
        AND (
            (check_in_date <= $2 AND check_out_date >= $2) OR
            (check_in_date <= $3 AND check_out_date >= $3) OR
            (check_in_date >= $2 AND check_out_date <= $3)
        )
        "#
    )
    .bind(property_id)
    .bind(check_in_date)
    .bind(check_out_date)
    .fetch_one(pool)
    .await?;

    let count: i64 = row.try_get("count")?;
    Ok(count == 0)
}

// Calculate total price for a booking
pub async fn calculate_booking_price(
    pool: &PgPool,
    property_id: Uuid,
    check_in_date: Date,
    check_out_date: Date,
    _guest_count: i32,
) -> Result<i32, AppError> {
    let property = get_property_by_id(pool, property_id).await?
        .ok_or_else(|| AppError::NotFound("Property not found".to_string()))?;

    let nights = (check_out_date - check_in_date).whole_days() as i32;
    let total_price = property.price_per_night * nights;

    Ok(total_price)
}
