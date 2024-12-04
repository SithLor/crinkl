pub struct Camera {
    pub CameraId: i32,
    pub Region: String,
    pub CameraLocation: RoadwayLocation,
    pub DisplayLatitude: f64,
    pub DisplayLongitude: f64,
    pub Title: String,
    pub Description: String,
    pub ImageURL: String,
    pub CameraOwner: String,
    pub OwnerURL: String,
    pub ImageWidth: i32,
    pub ImageHeight: i32,
    pub IsActive: bool,
    pub SortOrder: i32,
}

impl Camera {
    // Getter and Setter for CameraId
    pub fn get_camera_id(&self) -> i32 {
        self.CameraId
    }

    pub fn set_camera_id(&mut self, id: i32) {
        self.CameraId = id;
    }

    // Getter and Setter for Region
    pub fn get_region(&self) -> &str {
        &self.Region
    }

    pub fn set_region(&mut self, region: String) {
        self.Region = region;
    }

    // Getter and Setter for CameraLocation
    pub fn get_camera_location(&self) -> &RoadwayLocation {
        &self.CameraLocation
    }

    pub fn set_camera_location(&mut self, location: RoadwayLocation) {
        self.CameraLocation = location;
    }

    // Getter and Setter for DisplayLatitude
    pub fn get_display_latitude(&self) -> f64 {
        self.DisplayLatitude
    }

    pub fn set_display_latitude(&mut self, latitude: f64) {
        self.DisplayLatitude = latitude;
    }

    // Getter and Setter for DisplayLongitude
    pub fn get_display_longitude(&self) -> f64 {
        self.DisplayLongitude
    }

    pub fn set_display_longitude(&mut self, longitude: f64) {
        self.DisplayLongitude = longitude;
    }

    // Getter and Setter for Title
    pub fn get_title(&self) -> &str {
        &self.Title
    }

    pub fn set_title(&mut self, title: String) {
        self.Title = title;
    }

    // Getter and Setter for Description
    pub fn get_description(&self) -> &str {
        &self.Description
    }

    pub fn set_description(&mut self, description: String) {
        self.Description = description;
    }

    // Getter and Setter for ImageURL
    pub fn get_image_url(&self) -> &str {
        &self.ImageURL
    }

    pub fn set_image_url(&mut self, url: String) {
        self.ImageURL = url;
    }

    // Getter and Setter for CameraOwner
    pub fn get_camera_owner(&self) -> &str {
        &self.CameraOwner
    }

    pub fn set_camera_owner(&mut self, owner: String) {
        self.CameraOwner = owner;
    }

    // Getter and Setter for OwnerURL
    pub fn get_owner_url(&self) -> &str {
        &self.OwnerURL
    }

    pub fn set_owner_url(&mut self, url: String) {
        self.OwnerURL = url;
    }

    // Getter and Setter for ImageWidth
    pub fn get_image_width(&self) -> i32 {
        self.ImageWidth
    }

    pub fn set_image_width(&mut self, width: i32) {
        self.ImageWidth = width;
    }

    // Getter and Setter for ImageHeight
    pub fn get_image_height(&self) -> i32 {
        self.ImageHeight
    }

    pub fn set_image_height(&mut self, height: i32) {
        self.ImageHeight = height;
    }

    // Getter and Setter for IsActive
    pub fn get_is_active(&self) -> bool {
        self.IsActive
    }

    pub fn set_is_active(&mut self, active: bool) {
        self.IsActive = active;
    }

    // Getter and Setter for SortOrder
    pub fn get_sort_order(&self) -> i32 {
        self.SortOrder
    }

    pub fn set_sort_order(&mut self, order: i32) {
        self.SortOrder = order;
    }
}