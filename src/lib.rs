pub mod auth;
pub mod config;
pub mod db;
pub mod error;
pub mod handlers;
pub mod models;
pub mod routes;

#[cfg(test)]
mod tests {
    use super::models::{UserRole, PropertyType, BookingStatus};

    #[test]
    fn test_user_role_serialization() {
        let role = UserRole::User;
        let serialized = serde_json::to_string(&role).unwrap();
        assert_eq!(serialized, "\"User\"");
        
        let role = UserRole::PropertyOwner;
        let serialized = serde_json::to_string(&role).unwrap();
        assert_eq!(serialized, "\"PropertyOwner\"");
        
        let role = UserRole::Admin;
        let serialized = serde_json::to_string(&role).unwrap();
        assert_eq!(serialized, "\"Admin\"");
    }

    #[test]
    fn test_property_type_serialization() {
        let property_type = PropertyType::Hotel;
        let serialized = serde_json::to_string(&property_type).unwrap();
        assert_eq!(serialized, "\"Hotel\"");
        
        let property_type = PropertyType::Hostel;
        let serialized = serde_json::to_string(&property_type).unwrap();
        assert_eq!(serialized, "\"Hostel\"");
        
        let property_type = PropertyType::Apartment;
        let serialized = serde_json::to_string(&property_type).unwrap();
        assert_eq!(serialized, "\"Apartment\"");
    }

    #[test]
    fn test_booking_status_serialization() {
        let status = BookingStatus::Pending;
        let serialized = serde_json::to_string(&status).unwrap();
        assert_eq!(serialized, "\"Pending\"");
        
        let status = BookingStatus::Approved;
        let serialized = serde_json::to_string(&status).unwrap();
        assert_eq!(serialized, "\"Approved\"");
        
        let status = BookingStatus::Denied;
        let serialized = serde_json::to_string(&status).unwrap();
        assert_eq!(serialized, "\"Denied\"");
        
        let status = BookingStatus::Cancelled;
        let serialized = serde_json::to_string(&status).unwrap();
        assert_eq!(serialized, "\"Cancelled\"");
        
        let status = BookingStatus::Completed;
        let serialized = serde_json::to_string(&status).unwrap();
        assert_eq!(serialized, "\"Completed\"");
    }
}

