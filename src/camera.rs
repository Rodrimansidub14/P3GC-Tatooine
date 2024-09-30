extern crate nalgebra as na;
use na::{Matrix4, Point3, Vector3};

pub struct Camera {
    pub position: Point3<f32>,
    pub target: Point3<f32>,
    pub up: Vector3<f32>,
    pub view_matrix: Matrix4<f32>,

    // Spherical coordinates for orbiting
    pub azimuth_angle: f32,
    pub polar_angle: f32,
    pub radius: f32,
}

impl Camera {
    // Updated constructor
    pub fn new(position: Point3<f32>, target: Point3<f32>, up: Vector3<f32>) -> Self {
        // Calculate the initial radius and angles for spherical coordinates
        let radius = (target - position).magnitude();
        let azimuth_angle = (position.z - target.z).atan2(position.x - target.x);
        let polar_angle = (position.y - target.y).asin();

        let mut camera = Camera {
            position,
            target,
            up,
            view_matrix: Matrix4::identity(),
            azimuth_angle,
            polar_angle,
            radius,
        };
        camera.update_view_matrix();
        camera
    }

    // Move left/right across the global X-axis
    pub fn move_right_global(&mut self, amount: f32) {
        // Global right vector is along the X-axis
        let right = Vector3::x();
        self.position += right * amount;
        self.update_spherical_from_cartesian(); // Keep spherical coordinates in sync
        self.update_view_matrix();
    }

    // Move up/down across the global Y-axis
    pub fn move_up_global(&mut self, amount: f32) {
        // Global up vector es a lo largo del eje Y
        let up = Vector3::y();
        self.position += up * amount;
    
        // Restringir el valor de la posición Y para que no sea negativo
        if self.position.y < 0.0 {
            self.position.y = 0.0;
        }
    
        self.update_spherical_from_cartesian(); // Mantener las coordenadas esféricas sincronizadas
        self.update_view_matrix();
    }
    
    // Update the spherical coordinates (radius, azimuth, polar) from the camera's position in Cartesian space
    fn update_spherical_from_cartesian(&mut self) {
        let direction = self.position - self.target;
        self.radius = direction.magnitude();
        self.azimuth_angle = direction.z.atan2(direction.x); // Azimuth angle
        self.polar_angle = direction.y.asin().clamp(-std::f32::consts::FRAC_PI_2, std::f32::consts::FRAC_PI_2); // Polar angle
    }

    // Orbit the camera based on azimuth (horizontal) and polar (vertical) changes
    pub fn orbit(&mut self, delta_azimuth: f32, delta_polar: f32) {
        // Actualizar los ángulos (azimuth y polar)
        self.azimuth_angle += delta_azimuth; // Órbita horizontal
    
        // Mantener la cámara en el hemisferio positivo, limitando el ángulo polar para evitar valores negativos en Y
        self.polar_angle = (self.polar_angle + delta_polar)
            .clamp(-std::f32::consts::FRAC_PI_2 + 0.1, std::f32::consts::FRAC_PI_2 - 0.1);
    
        // Actualizar la posición de la cámara
        self.update_position();
    }
    
    // Zoom functionality
    pub fn zoom(&mut self, zoom_amount: f32) {
        const MIN_RADIUS: f32 = 0.001;
        const MAX_RADIUS: f32 = 150.0;
        self.radius = (self.radius - zoom_amount).clamp(MIN_RADIUS, MAX_RADIUS);
        self.update_position();
    }

    // Update the camera position based on spherical coordinates
    pub fn update_position(&mut self) {
        // Lock del target de la cámara al origen (0,0,0)
        self.target = Point3::new(0.0, 0.0, 0.0);
    
        // Calcular la nueva posición usando coordenadas esféricas
        self.position.x =
            self.target.x + self.radius * self.polar_angle.cos() * self.azimuth_angle.cos();
        self.position.y = self.target.y + self.radius * self.polar_angle.sin();
        
        // Restringir el eje Y para que solo sea positivo
        if self.position.y < 0.0 {
            self.position.y = 0.0;
        }
    
        self.position.z =
            self.target.z + self.radius * self.polar_angle.cos() * self.azimuth_angle.sin();
        
        self.update_view_matrix();
    }
    
    // Update the view matrix
    pub fn update_view_matrix(&mut self) {
        self.view_matrix = Matrix4::look_at_rh(&self.position, &self.target, &self.up);
    }

    // Transforms a direction vector using the view matrix of the camera
    pub fn transform_direction(&self, direction: &Vector3<f32>) -> Vector3<f32> {
        let transformed_direction = self.view_matrix.transform_vector(direction);
        transformed_direction.normalize()
    }
}
