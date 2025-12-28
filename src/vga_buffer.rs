use volatile::Volatile;
use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;

// VGA buffer implemetation
const VGA_WIDTH: usize = 80;
const VGA_HEIGHT: usize = 25;
const ASCII_PRINTABLE_START: u8 = 0x20;
const ASCII_PRINTABLE_END: u8 = 0x7e;
const ASCII_UNPRINTABLE: u8 = 0xfe;

// VGA buffer map
const VGA_ADDRESS: usize = 0xb8000;

// global Writer instance
// since a reference (&mut Buffer) can only be validated at compile time, need a lazy static instance (inits at runtime)
// since this static is currently immutable and not usable for writing, we can wrap this in Mutex
// spinlocks for now...
lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        row_pos: 0,
        col_pos: 0,
        color_code: ColorCode::new(Color::LightRed, Color::Black),
        buffer: unsafe { &mut *(VGA_ADDRESS as *mut Buffer) },
    });
}


// impl fmt::Write enables Rust-level macros for string-formatting
// required method: `write_str`
impl fmt::Write for Writer
{
    fn write_str(&mut self, s: &str) -> fmt::Result
    {
        self.write_string(s);   // write_str is now just a wrapper for write_string()
        Ok(())
    }
}

// redefining print and println macros to use our implementation of fmt::Write trait
#[macro_export]
macro_rules! print 
{
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println 
{
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;

    // disable interrupts when writing
    use x86_64::instructions::interrupts;

    interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    })
}


#[allow(dead_code)]
#[repr(u8)]     // unsigned 8-bit representation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color
{
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}


#[allow(dead_code)]
#[repr(transparent)]    
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ColorCode(u8);

impl ColorCode
{
    fn new(fg: Color, bg: Color) -> ColorCode
    {
        ColorCode(((bg as u8) << 4) | (fg as u8))
    }
}


// ScreenChar struct (encodes a character and its color)
// 16-bit C struct
#[allow(dead_code)]
#[repr(C)]    
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ScreenChar
{
    char: u8,
    color_code: ColorCode
}


#[repr(transparent)]
struct Buffer
{
    // define a VGA buffer of (VGA_WIDTH x VGA_HEIGHT), type ScreenChar
    // marking as volatile to prevent compiler optimizing away screen writes (reducing side effects)
    buf: [[Volatile<ScreenChar>; VGA_WIDTH]; VGA_HEIGHT]
}


// Write struct (manages VGA output logic)
// Writer is a controlled object (atomic?)
pub struct Writer
{
    row_pos: usize,
    col_pos: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,    // valid reference for whole runtime
}

impl Writer
{
    // write a byte to the buffer
    pub fn write_byte(&mut self, byte: u8)
    {
        match byte 
        {
            b'\n' => self.new_line(),     // print newline

            // else, its a byte, add posito logic
            byte => {
                if self.col_pos >= VGA_WIDTH
                {
                    // add a newline
                    self.new_line();
                }

                // get postion
                let row: usize = self.row_pos;
                let col: usize = self.col_pos;

                // get color code
                let color_code = self.color_code;

                // add entry to the buffer 
                self.buffer.buf[row][col].write(ScreenChar { char: byte, color_code });

                // increment col
                self.col_pos += 1;
            } 
        }
    }

    // write a string to the buffer (series of bytes)
    pub fn write_string(&mut self, string: &str)
    {
        for byte in string.bytes()
        {
            match byte 
            {
                // check if this byte is printable
                b'\n' | (ASCII_PRINTABLE_START..ASCII_PRINTABLE_END) => self.write_byte(byte),

                // else...
                _ => self.write_byte(ASCII_UNPRINTABLE),
            }
        }
    }

    fn new_line(&mut self)  
    {   
        // check if we need to scroll
        if self.row_pos >= VGA_HEIGHT - 1
        {
            // shift all rows up by one
            for row in 1..VGA_HEIGHT 
            {
                for col in 0..VGA_WIDTH 
                {
                    let character = self.buffer.buf[row][col].read();
                    self.buffer.buf[row - 1][col].write(character);
                }
            }

            // clear out the last row
            self.clear_row(VGA_HEIGHT - 1);
        }
        else
        {
            // just move to next row
            self.row_pos += 1;
        }

        // reset col_pos
        self.col_pos = 0;
    }

    fn clear_row (&mut self, row: usize)
    {
        for col in 0..VGA_WIDTH
        {
            self.buffer.buf[row][col].write(ScreenChar { char: b' ', color_code: self.color_code } );
        }
    }
}


// test functions...
// pub fn splash_screen_no_println()
// {
//     use core::fmt::Write;
    
//     WRITER.lock().write_str("================================================================================").unwrap();
//     WRITER.lock().write_byte(b'\n');
//     WRITER.lock().write_str("                               WELCOME TO FERRIX                                ").unwrap();
//     WRITER.lock().write_byte(b'\n');
//     WRITER.lock().write_str("================================================================================").unwrap();
//     WRITER.lock().write_byte(b'\n');
//     WRITER.lock().write_byte(b'\n');
    
//     WRITER.lock().write_str("   [OK] VGA Buffer initialized").unwrap();
//     WRITER.lock().write_byte(b'\n');
//     WRITER.lock().write_str("   [OK] Kernel loaded successfully").unwrap();
//     WRITER.lock().write_byte(b'\n');
//     WRITER.lock().write_byte(b'\n');
    
//     WRITER.lock().write_str("--------------------------------------------------------------------------------").unwrap();
//     WRITER.lock().write_byte(b'\n');
//     write!(WRITER.lock(), "Numbers test: {} {}", 42, 1.337).unwrap();
//     WRITER.lock().write_byte(b'\n');

//     println!("Hello Ferrix {}", "!");
// }

pub fn splash_screen()
{
    println!(" ============================================================================== ");
    println!("                           WELCOME TO FERRIX (v0.1.0)                           ");
    println!(" ============================================================================== ");
    println!();
    println!("   [OK] VGA Buffer initialized");
    println!("   [OK] Kernel loaded successfully");
    println!();
    println!(" ------------------------------------------------------------------------------ ");
}




// ---------- TESTS ----------
#[test_case]
fn test_println_simple() 
{
    println!("test_println_simple output");
}

#[test_case]
fn test_println_many() 
{
    for _ in 0..200 {
        println!("test_println_many output");
    }
}

#[test_case]
fn test_println_output() {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    let s = "Some test string that fits on a single line";
    interrupts::without_interrupts(|| {
        let mut writer = WRITER.lock();
        writeln!(writer, "\n{}", s).expect("writeln failed");
        for (i, c) in s.chars().enumerate() {
            let screen_char = writer.buffer.buf[VGA_HEIGHT - 2][i].read();
            assert_eq!(char::from(screen_char.char), c);
        }
    });
}