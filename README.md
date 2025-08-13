# Airbnb-like Reservation System Backend

A production-grade Rust API backend built with Actix-web for an Airbnb-like property reservation system. This system supports three user roles: Users, Property Owners, and Admins.

## Features

### 🏠 Property Management
- **Property Types**: Hotel, Hostel, Apartment
- **Rich Property Details**: Location, pricing, amenities, images, capacity
- **Advanced Filtering**: By location, price, property type, guest capacity, dates
- **Pagination**: Efficient browsing with configurable page sizes

### 👥 User Management
- **Three User Roles**:
  - **User**: Browse properties, make bookings, manage reservations
  - **Property Owner**: List properties, manage bookings, approve/deny requests
  - **Admin**: Manage property owners, system oversight
- **Secure Authentication**: JWT-based with bcrypt password hashing
- **Role-based Access Control**: Granular permissions for different operations

### 📅 Booking System
- **Smart Availability**: Date conflict detection and availability checking
- **Booking Workflow**: Pending → Approved/Denied → Completed/Cancelled
- **Flexible Cancellation**: Users can cancel their bookings, owners can cancel property bookings
- **Price Calculation**: Automatic total price calculation based on nights and guest count

### 🛡️ Security & Production Features
- **Input Validation**: Comprehensive request validation using validator crate
- **Error Handling**: Structured error responses with appropriate HTTP status codes
- **CORS Support**: Configurable cross-origin resource sharing
- **Logging**: Structured logging with tracing
- **Database Optimization**: Proper indexing and efficient queries
- **Connection Pooling**: Efficient database connection management

## Tech Stack

- **Framework**: Actix-web 4.x
- **Database**: PostgreSQL with SQLx
- **Authentication**: JWT tokens with bcrypt
- **Validation**: Validator crate
- **Error Handling**: Thiserror + custom error types
- **Logging**: Tracing + tracing-subscriber
- **Async Runtime**: Tokio

## Project Structure

```
airbnb-backend/
├── src/
│   ├── main.rs          # Application entry point
│   ├── models.rs         # Data models and DTOs
│   ├── auth.rs           # Authentication and authorization
│   ├── db.rs            # Database operations
│   ├── handlers.rs       # HTTP request handlers
│   ├── routes.rs         # Route configuration
│   ├── error.rs          # Error handling
│   └── config.rs         # Configuration management
├── migrations/           # Database migrations
├── Cargo.toml           # Dependencies and project metadata
└── README.md            # This file
```

## Prerequisites

- Rust 1.70+ (edition 2021)
- PostgreSQL 12+
- Cargo (comes with Rust)

## Setup Instructions

### 1. Clone and Navigate
```bash
git clone <repository-url>
cd airbnb-backend
```

### 2. Environment Configuration
Create a `.env` file in the project root:
```env
DATABASE_URL=postgresql://username:password@localhost:5432/airbnb_db
HOST=127.0.0.1
PORT=8080
RUST_LOG=debug
```

### 3. Database Setup
```bash
# Create database
createdb airbnb_db

# Run migrations
psql -d airbnb_db -f migrations/001_initial_schema.sql
```

### 4. Build and Run
```bash
# Build the project
cargo build --release

# Run the server
cargo run
```

The server will start at `http://127.0.0.1:8080`

## API Endpoints

### Public Endpoints (No Authentication)

#### Authentication
- `POST /api/v1/auth/register` - User registration
- `POST /api/v1/auth/login` - User login

#### Property Browsing
- `GET /api/v1/properties` - List properties with filters and pagination
- `GET /api/v1/properties/{id}` - Get property details

### Protected Endpoints (Authentication Required)

#### User Operations
- `POST /api/v1/bookings` - Create a booking request
- `GET /api/v1/bookings` - View user's bookings
- `POST /api/v1/bookings/{id}/cancel` - Cancel a booking

#### Property Owner Operations
- `POST /api/v1/properties` - Create a new property listing
- `GET /api/v1/properties/my` - View owner's properties
- `GET /api/v1/properties/{id}/bookings` - View bookings for a property
- `POST /api/v1/bookings/{id}/approve` - Approve a booking request
- `POST /api/v1/bookings/{id}/deny` - Deny a booking request

#### Admin Operations
- `POST /api/v1/admin/property-owners` - Create a new property owner account

## API Usage Examples

### User Registration
```bash
curl -X POST http://localhost:8080/api/v1/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "email": "user@example.com",
    "password": "password123",
    "first_name": "John",
    "last_name": "Doe"
  }'
```

### User Login
```bash
curl -X POST http://localhost:8080/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "user@example.com",
    "password": "password123"
  }'
```

### Browse Properties
```bash
curl "http://localhost:8080/api/v1/properties?location=Paris&max_guests=4&page=1&per_page=10"
```

### Create Property (Property Owner)
```bash
curl -X POST http://localhost:8080/api/v1/properties \
  -H "Authorization: Bearer YOUR_JWT_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Cozy Paris Apartment",
    "description": "Beautiful apartment in the heart of Paris",
    "property_type": "apartment",
    "location": "Paris, France",
    "address": "123 Rue de la Paix",
    "city": "Paris",
    "country": "France",
    "postal_code": "75001",
    "price_per_night": 15000,
    "max_guests": 4,
    "bedrooms": 2,
    "bathrooms": 1,
    "amenities": ["WiFi", "Kitchen", "Washing Machine"],
    "images": ["image1.jpg", "image2.jpg"]
  }'
```

### Create Booking
```bash
curl -X POST http://localhost:8080/api/v1/bookings \
  -H "Authorization: Bearer YOUR_JWT_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "property_id": "property-uuid-here",
    "guest_count": 2,
    "special_requests": "Early check-in if possible"
  }'
```

## Database Schema

### Users Table
- `id`: UUID primary key
- `email`: Unique email address
- `password_hash`: Bcrypt hashed password
- `first_name`, `last_name`: User names
- `role`: User role (user, property_owner, admin)
- `is_active`: Account status
- `created_at`, `updated_at`: Timestamps

### Properties Table
- `id`: UUID primary key
- `owner_id`: Reference to users table
- `title`, `description`: Property details
- `property_type`: Hotel, Hostel, or Apartment
- `location`, `address`, `city`, `country`, `postal_code`: Location info
- `latitude`, `longitude`: GPS coordinates
- `price_per_night`: Price in cents
- `max_guests`, `bedrooms`, `bathrooms`: Capacity info
- `amenities`, `images`: Arrays of amenities and image URLs
- `is_active`: Property availability status

### Bookings Table
- `id`: UUID primary key
- `property_id`, `user_id`: References to properties and users
- `check_in_date`, `check_out_date`: Stay dates
- `total_price`: Total cost in cents
- `status`: Booking status (pending, approved, denied, cancelled, completed)
- `guest_count`: Number of guests
- `special_requests`: Optional special requirements

## Security Features

- **Password Hashing**: Bcrypt with configurable cost
- **JWT Authentication**: Secure token-based authentication
- **Role-based Access Control**: Granular permissions for different operations
- **Input Validation**: Comprehensive request validation
- **SQL Injection Prevention**: Parameterized queries with SQLx
- **CORS Configuration**: Configurable cross-origin policies

## Performance Features

- **Database Indexing**: Optimized queries with proper indexes
- **Connection Pooling**: Efficient database connection management
- **Pagination**: Configurable page sizes for large datasets
- **Efficient Queries**: Optimized SQL with proper joins and filtering

## Development

### Running Tests
```bash
cargo test
```

### Code Formatting
```bash
cargo fmt
```

### Linting
```bash
cargo clippy
```

### Database Migrations
```bash
# Generate new migration
cargo sqlx migrate add migration_name

# Run migrations
cargo sqlx migrate run

# Revert migrations
cargo sqlx migrate revert
```

## Production Deployment

### Environment Variables
- `DATABASE_URL`: PostgreSQL connection string
- `HOST`: Server host (default: 127.0.0.1)
- `PORT`: Server port (default: 8080)
- `RUST_LOG`: Logging level (default: debug)

### Docker Deployment
```dockerfile
FROM rust:1.70 as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y libpq5 && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/app/target/release/airbnb-backend /usr/local/bin/
CMD ["airbnb-backend"]
```

### Health Checks
The API includes built-in health monitoring through logging and proper error handling.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Run the test suite
6. Submit a pull request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Support

For questions and support, please open an issue in the GitHub repository.
