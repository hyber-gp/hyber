use crate::util::Vector2D;

/// Structure that contains optional settings for a display window.
/// It should be used when creating a new Display.
/// It also provides default settings for a new Display.
pub struct DisplayDescritor {
    /// Indicates if display has a border (default: true)
    pub border: bool,
    /// Indicates if display is titled (default: true)
    pub titled: bool,
    /// Indicates if display is resizable (default: false)
    pub resizable: bool,
    /// Indicates if display always appears on top of all displays which are not topmost (default: false)
    pub topmost: bool,
    /// Indicates if display is minimizable (default: true)
    pub minimizable: bool,
}

impl DisplayDescritor {
    /// Returns a DisplayDescriptor with default values, allowing to override each attribute
    ///
    /// # Arguments
    ///
    /// # Examples
    ///
    /// ```no_run
    /// let mut display_descriptor = DisplayDescriptor { border: false, titled: false, ..Default::default() }
    /// ```
    ///
    /// ```no_run
    /// let mut display_descriptor = DisplayDescriptor { ..Default::default() }
    /// ```
    pub fn default() -> DisplayDescritor {
        DisplayDescritor {
            border: true,
            titled: true,
            resizable: false,
            topmost: false,
            minimizable: true,
        }
    }
}

/// Implements the necessary methods to a complete Display/Window system
pub trait Display {
    type Buffer;

    /// Creates and opens up a new display
    ///
    /// # Arguments
    /// * `title` - Title of the display
    /// * `width` - Width of the display
    /// * `height` - Height of the display
    /// * `display_descriptor` - Holds a reference to a DisplayDescriptor, which contains optional stylings for the display
    ///
    /// # Examples
    ///
    /// Create and open up a display with popup behaviour
    ///
    /// ```no_run
    /// impl Display for Implementor {
    ///     fn new(title: &str, width: usize, height: usize, display_descriptor: DisplayDescritor) -> Self {
    ///         ...
    ///     }
    /// }
    /// ```
    ///
    /// ```no_run
    /// let mut display = Implementor::new("Example", 640, 400,
    ///     DisplayDescriptor {
    ///         topmost: true,
    ///         minimizable: false,
    ///         ..DisplayDescriptor::default()
    /// });
    /// ```
    fn new(title: &str, width: usize, height: usize, display_descriptor: DisplayDescritor) -> Self;

    /// Sets a new title for the display
    ///
    /// # Arguments
    ///
    /// * `title` - Title of the display
    ///
    /// # Examples
    ///
    /// ```no_run
    /// impl Display for Implementor {
    ///     fn set_title(&mut self, title: &str) {
    ///         ...
    ///     }
    /// }
    /// ```
    ///
    /// ```no_run
    /// let mut display = Implementor::new("Example", 640, 400, DisplayDescriptor::default());
    ///
    /// display.set_title("Other Example");
    /// ```
    fn set_title(&mut self, title: &str);

    /// Updates the display
    ///
    /// # Arguments
    ///
    /// # Examples
    ///
    /// Updates the display after setting a new title for it
    ///
    /// ```no_run
    /// impl Display for Implementor {
    ///     fn update(&mut self) {
    ///         ...
    ///     }
    /// }
    /// ```
    ///
    /// ```no_run
    /// let mut display = Implementor::new("Example", 640, 400, DisplayDescriptor::default());
    ///
    /// display.set_title("Other Example");
    ///
    /// display.update();
    /// ```
    fn update(&mut self);

    /// Updates the display along with its buffer
    ///
    /// # Arguments
    /// * `buffer` - buffer associated with the display
    /// * `width` - width of the buffer
    /// * `height` - height of the buffer
    ///
    /// # Examples
    /// 
    /// Updates the display along with its buffer after setting the background color to green
    ///
    /// ```no_run
    /// impl Display for Implementor {
    ///     fn update_with_buffer(&mut self, buffer: &Self::Buffer, width: usize, height: usize) {
    ///         ...
    ///     }
    /// }
    /// ```
    ///
    /// ```no_run
    /// 
    /// let mut display = Implementor::new("Example", 640, 400, DisplayDescriptor::default());
    ///
    /// display.set_background_color(0, 255, 0);
    /// 
    /// (width, height) = display.get_size();
    /// 
    /// let mut buffer: buffer: Vec<u32> = vec![0x00FF00; width * height];
    ///
    /// display.update_with_buffer(buffer, width, height);
    /// ```
    fn update_with_buffer(&mut self, buffer: &Self::Buffer, width: usize, height: usize);

    /// Check if the display is open. The user may want to take some action depending on this state.
    ///
    /// # Arguments
    ///
    /// # Examples
    ///
    /// ```no_run
    /// impl Display for Implementor {
    ///     fn is_open(&self) -> bool {
    ///         ...
    ///     }
    /// }
    /// ```
    ///
    /// ```no_run
    /// let mut display = Implementor::new("Example", 640, 400, DisplayDescriptor::default());
    ///
    /// let display_opened = display.is_open();
    /// ```
    fn is_open(&self) -> bool;

    /// Sets the position of the window, relative to the topleft corner of the display.
    ///
    /// # Arguments
    /// * `x` - x-coordinate for the topleft corner
    /// * `y` - y-coordinate for the topleft corner
    ///
    /// # Examples
    ///
    /// ```no_run
    /// impl Display for Implementor {
    ///     fn set_position(&mut self, x: usize, y: usize) {
    ///         ...
    ///     }
    /// }
    /// ```
    ///
    /// ```no_run
    /// let mut display = Implementor::new("Example", 640, 400, DisplayDescriptor::default());
    ///
    /// display.set_position(0, 0);
    /// ```
    fn set_position(&mut self, x: usize, y: usize);

    /// Toggles a border in the display
    ///
    /// # Arguments
    /// * `border` - boolean indicating if display has a border
    ///
    /// # Examples
    ///
    /// Make the display borderless
    ///
    /// ```no_run
    /// impl Display for Implementor {
    ///     fn border(&mut self, border: bool) {
    ///         ...
    ///     }
    /// }
    /// ```
    ///
    /// ```no_run
    /// let mut display = Implementor::new("Example", 640, 400, DisplayDescriptor::default());
    ///
    /// display.border(false);
    /// ```
    fn border(&mut self, border: bool);

    /// Makes the display resizable
    ///
    /// # Arguments
    /// * `resizable` - boolean indicating if display is resizable
    ///
    /// # Examples
    ///
    /// ```no_run
    /// impl Display for Implementor {
    ///     fn resizable(&mut self, resizable: bool) {
    ///         ...
    ///     }
    /// }
    /// ```
    ///
    /// ```no_run
    /// let mut display = Implementor::new("Example", 640, 400, DisplayDescriptor::default());
    ///
    /// display.resizable(true);
    /// ```
    fn resizable(&mut self, resizable: bool);

    /// Makes the display always appear on top of those that are not topmost
    ///
    /// # Arguments
    /// * `topmost` - boolean indicating if display is topmost
    ///
    /// # Examples
    ///
    /// ```no_run
    /// impl Display for Implementor {
    ///     fn topmost(&mut self, topmost: bool) {
    ///         ...
    ///     }
    /// }
    /// ```
    ///
    /// ```no_run
    /// let mut display = Implementor::new("Example", 640, 400, DisplayDescriptor::default());
    ///
    /// display.topmost(true);
    /// ```
    fn topmost(&mut self, topmost: bool);

    /// Makes the display minimizable
    ///
    /// # Arguments
    /// * `minimizable` - boolean indicating if display is minimizable
    ///
    /// # Examples
    ///
    /// Makes the display non-minimizable
    ///
    /// ```no_run
    /// impl Display for Implementor {
    ///     fn minimizable(&mut self, minimizable: bool) {
    ///         ...
    ///     }
    /// }
    /// ```
    ///
    /// ```no_run
    /// let mut display = Implementor::new("Example", 640, 400, DisplayDescriptor::default());
    ///
    /// display.minimizable(false);
    /// ```
    fn minimizable(&mut self, minimizable: bool);

    /// Sets background color that is updated in the buffer.
    ///
    /// # Arguments
    /// * `red` - value for red color
    /// * `green` - value value for green color
    /// * `blue` - value for blue color
    ///
    /// # Examples
    ///
    /// Set background color to yellow
    ///
    /// ```no_run
    /// impl Display for Implementor {
    ///     fn set_background_color(&mut self, red: usize, green: usize, blue: usize) {
    ///         ...
    ///     }
    /// }
    /// ```
    ///
    /// ```no_run
    /// let mut display = Implementor::new("Example", 640, 400, DisplayDescriptor::default());
    ///
    /// display.set_background_color(255, 255, 0);
    /// ```
    fn set_background_color(&mut self, red: usize, green: usize, blue: usize);

    /// Returns the current size of the display
    ///
    /// # Arguments
    ///
    /// # Examples
    ///
    /// ``no_run
    /// impl Display for Implementor {
    ///     fn get_size(&self) -> Vector2D {
    ///         ...
    ///     }
    /// }
    /// ```
    ///
    /// ```no_run
    /// let mut display = Implementor::new("Example", 640, 400, DisplayDescriptor::default());
    ///
    /// let (width, height) = display.get_size();
    /// ```
    fn get_size(&self) -> Vector2D;

    /// Checks if the display is the current active one
    ///
    /// # Arguments
    ///
    /// # Examples
    ///
    /// ``no_run
    /// impl Display for Implementor {
    ///     fn is_active(&mut self) -> bool {
    ///         ...
    ///     }
    /// }
    /// ```
    ///
    /// ```no_run
    /// let mut display = Implementor::new("Example", 640, 400, DisplayDescriptor::default());
    ///
    /// let display_active = display.is_active();
    /// ```
    fn is_active(&mut self) -> bool;
}
