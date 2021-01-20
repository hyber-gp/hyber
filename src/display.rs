use crate::util::Vector2D;

/// Optional display's settings that should be used when creating a new Display
///
/// It also provides default settings for a new Display
pub struct DisplayDescritor {
    /// Whether the display has a border
    ///
    /// [default: true]
    pub border: bool,

    /// Whether the display is titled
    ///
    /// [default: true]
    pub titled: bool,
    
    /// Whether the display is resizable
    ///
    /// [default: false]
    pub resizable: bool,
    
    /// Whether the display always appears on top of all displays which are not topmost
    ///
    /// [default: false]
    pub topmost: bool,
    
    /// Whether the display is minimizable
    ///
    /// [default: true]
    pub minimizable: bool,
}

impl DisplayDescritor {
    /// Creates a new default `DisplayDescritor`
    ///
    /// # Returns
    /// A new DisplayDescriptor with default values
    ///
    /// # Arguments
    /// No arguments
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

/// Display is the window that are presented to the user. The widgets are 
// rendered within this display, what makes possible to render the user interface
pub trait Display {
    /// Data buffer to be displayed on the window
    type Buffer;

    /// Creates and present a new `Display`
    ///
    /// # Returns
    /// The display created and already presented
    ///
    /// # Arguments
    /// * `title` - the title to be assigned to the display
    /// * `width` - the width to be assigned to the display
    /// * `height` - the height to be assigned to the display
    /// * `display_descriptor` - the reference to a `DisplayDescriptor` which contains
    /// optional stylings to be assigned to the display
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
    /// # Returns
    /// No returns
    ///
    /// # Arguments
    /// * `title` - the title to be assigned to the display
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
    /// # Returns
    /// No returns
    ///
    /// # Arguments
    /// No arguments
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
    /// # Returns
    /// No returns
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

    /// Checks if the display is open
    ///
    /// # Returns
    /// True, if the display is opened, false otherwise
    ///
    /// # Arguments
    /// No arguments
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

    /// Sets display's position, relative to the top left corner
    ///
    /// # Returns
    /// No returns
    ///
    /// # Arguments
    /// * `x` - x-coordinate to be assigned to the display's top left corner
    /// * `y` - y-coordinate to be assigned to the display's top left corner
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

    /// Sets the display's border property
    ///
    /// # Returns
    /// No returns
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

    /// Sets the display's resizable property
    ///
    /// # Returns
    /// No returns
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
    /// # Returns
    /// No returns
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

    /// Sets the display's minimizable property
    ///
    /// # Returns
    /// No returns
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

    /// Sets the display's background color
    ///
    /// # Returns
    /// No returns
    ///
    /// # Arguments
    /// * `red` - value for red color
    /// * `green` - value for green color
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

    /// Gets display's current size
    ///
    /// # Returns
    /// The current size of the display
    ///
    /// # Arguments
    /// No arguments
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
    /// # Returns
    /// True, if the display is the current active one, false otherwise
    ///
    /// # Arguments
    /// No arguments
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
