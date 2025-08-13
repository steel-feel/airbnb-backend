use crate::handlers::*;
use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    // Public routes (no authentication required)
    cfg.service(
        web::scope("/api/v1")
            .route("/auth/register", web::post().to(register))
            .route("/auth/login", web::post().to(login))
            .route("/properties", web::get().to(get_properties))
            .route("/properties/{id}", web::get().to(get_property))
    )
    
    // Protected routes (authentication required)
    .service(
        web::scope("/api/v1")
            .route("/bookings", web::post().to(create_booking))
            .route("/bookings", web::get().to(get_my_bookings))
            .route("/bookings/{id}/cancel", web::post().to(cancel_booking))
            
            // Property owner routes
            .route("/properties", web::post().to(create_property))
            .route("/properties/my", web::get().to(get_my_properties))
            .route("/properties/{id}/bookings", web::get().to(get_property_bookings))
            .route("/bookings/{id}/approve", web::post().to(approve_booking))
            .route("/bookings/{id}/deny", web::post().to(deny_booking))
            
            // Admin routes
            .route("/admin/property-owners", web::post().to(create_property_owner))
    );
}
