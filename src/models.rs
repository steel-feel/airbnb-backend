use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::{Date, OffsetDateTime};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub first_name: String,
    pub last_name: String,
    #[sqlx(rename = "role")]
    pub role: UserRole,
    pub is_active: bool,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq, Eq)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
pub enum UserRole {
    User,
    PropertyOwner,
    Admin,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Property {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub title: String,
    pub description: String,
    #[sqlx(rename = "property_type")]
    pub property_type: PropertyType,
    pub location: String,
    pub address: String,
    pub city: String,
    pub country: String,
    pub postal_code: String,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub price_per_night: i32, // in cents
    pub max_guests: i32,
    pub bedrooms: i32,
    pub bathrooms: i32,
    pub amenities: Vec<String>,
    pub images: Vec<String>,
    pub is_active: bool,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq, Eq)]
#[sqlx(type_name = "property_type", rename_all = "lowercase")]
pub enum PropertyType {
    Hotel,
    Hostel,
    Apartment,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PropertyAvailability {
    pub id: Uuid,
    pub property_id: Uuid,
    pub date: Date,
    pub is_available: bool,
    pub price_override: Option<i32>, // in cents
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Booking {
    pub id: Uuid,
    pub property_id: Uuid,
    pub user_id: Uuid,
    pub check_in_date: Date,
    pub check_out_date: Date,
    pub total_price: i32, // in cents
    #[sqlx(rename = "status")]
    pub status: BookingStatus,
    pub guest_count: i32,
    pub special_requests: Option<String>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq, Eq)]
#[sqlx(type_name = "booking_status", rename_all = "lowercase")]
pub enum BookingStatus {
    Pending,
    Approved,
    Denied,
    Cancelled,
    Completed,
}

// DTOs for API requests/responses
#[derive(Debug, Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
    #[validate(length(min = 1))]
    pub first_name: String,
    #[validate(length(min = 1))]
    pub last_name: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 1))]
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: UserResponse,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub role: UserRole,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreatePropertyRequest {
    #[validate(length(min = 1))]
    pub title: String,
    #[validate(length(min = 1))]
    pub description: String,
    pub property_type: PropertyType,
    #[validate(length(min = 1))]
    pub location: String,
    #[validate(length(min = 1))]
    pub address: String,
    #[validate(length(min = 1))]
    pub city: String,
    #[validate(length(min = 1))]
    pub country: String,
    #[validate(length(min = 1))]
    pub postal_code: String,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    #[validate(range(min = 1))]
    pub price_per_night: i32,
    #[validate(range(min = 1))]
    pub max_guests: i32,
    #[validate(range(min = 1))]
    pub bedrooms: i32,
    #[validate(range(min = 1))]
    pub bathrooms: i32,
    pub amenities: Vec<String>,
    pub images: Vec<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateBookingRequest {
    pub property_id: Uuid,
    #[validate(range(min = 1))]
    pub guest_count: i32,
    pub special_requests: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct PropertyFilters {
    pub location: Option<String>,
    pub property_type: Option<PropertyType>,
    pub min_price: Option<i32>,
    pub max_price: Option<i32>,
    pub max_guests: Option<i32>,
    pub check_in_date: Option<Date>,
    pub check_out_date: Option<Date>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
    pub total_pages: i64,
}

#[derive(Debug, Serialize)]
pub struct PropertyResponse {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub property_type: PropertyType,
    pub location: String,
    pub address: String,
    pub city: String,
    pub country: String,
    pub price_per_night: i32,
    pub max_guests: i32,
    pub bedrooms: i32,
    pub bathrooms: i32,
    pub amenities: Vec<String>,
    pub images: Vec<String>,
    pub owner: UserResponse,
}

#[derive(Debug, Serialize)]
pub struct BookingResponse {
    pub id: Uuid,
    pub property: PropertyResponse,
    pub user: UserResponse,
    pub check_in_date: Date,
    pub check_out_date: Date,
    pub total_price: i32,
    pub status: BookingStatus,
    pub guest_count: i32,
    pub special_requests: Option<String>,
    pub created_at: OffsetDateTime,
}
