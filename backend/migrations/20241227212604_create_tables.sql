-- Table for garages
CREATE TABLE garages (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    location TEXT NOT NULL,
    city TEXT NOT NULL,
    capacity INTEGER NOT NULL
);

-- Table for cars
CREATE TABLE cars (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    make TEXT NOT NULL,
    model TEXT NOT NULL,
    production_year INTEGER NOT NULL,
    license_plate TEXT UNIQUE NOT NULL
);

-- Many-to-Many relationship between cars and garages
CREATE TABLE car_garages (
    car_id TEXT NOT NULL REFERENCES cars(id) ON DELETE CASCADE, -- Uuid as TEXT
    garage_id TEXT NOT NULL REFERENCES garages(id) ON DELETE CASCADE, -- Uuid as TEXT
    PRIMARY KEY (car_id, garage_id)
);

-- Table for maintenance requests
CREATE TABLE maintenance_requests (
    id TEXT PRIMARY KEY, -- Uuid as TEXT
    car_id TEXT NOT NULL REFERENCES cars(id) ON DELETE CASCADE, -- Uuid as TEXT
    garage_id TEXT NOT NULL REFERENCES garages(id) ON DELETE CASCADE, -- Uuid as TEXT
    service_type TEXT NOT NULL,
    scheduled_date TEXT NOT NULL,
    created_at TEXT DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE maintenance (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    car_id TEXT NOT NULL,       -- Changed to TEXT
    garage_id TEXT NOT NULL,    -- Changed to TEXT
    service_type TEXT NOT NULL,
    scheduled_date TEXT NOT NULL
);