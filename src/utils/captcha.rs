use std::io::Cursor;

use base64::engine::general_purpose;
use base64::Engine;
use image::DynamicImage;
use image::ImageOutputFormat::Jpeg;
use image::{ImageBuffer, Rgb};
use imageproc::drawing::{draw_cubic_bezier_curve_mut, draw_hollow_ellipse_mut, draw_text_mut};
use imageproc::noise::{gaussian_noise_mut, salt_and_pepper_noise_mut};
use rand::{thread_rng, Rng};
use rusttype::{Font, Scale};

use crate::utils::captcha;
///
/// 验证码生成器
///

// Define the verification code characters.
// Remove 0, O, I, L and other easily confusing letters
pub const BASIC_CHAR: [char; 54] = [
	'2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'J', 'K', 'M',
	'N', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g',
	'h', 'j', 'k', 'm', 'n', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
];

// Define a random color for a string
pub const LIGHT_BASIC_COLOR: [[u8; 3]; 5] = [
	[214, 14, 50],
	[240, 181, 41],
	[176, 203, 40],
	[105, 137, 194],
	[242, 140, 71],
];
pub const DARK_BASIC_COLOR: [[u8; 3]; 5] = [
	[251, 188, 5],
	[116, 192, 255],
	[255, 224, 133],
	[198, 215, 97],
	[247, 185, 168],
];

// Define background color
pub const LIGHT: [u8; 3] = [224, 238, 253];
pub const DARK: [u8; 3] = [18, 18, 18];

// Define font size
pub const SCALE_SM: Scale = Scale { x: 38.0, y: 35.0 };
pub const SCALE_MD: Scale = Scale { x: 45.0, y: 42.0 };
pub const SCALE_LG: Scale = Scale { x: 53.0, y: 50.0 };

/***
 * Generate random numbers
 * params num - maximum random number
 */
pub fn get_rnd(num: usize) -> usize {
	let mut rng = thread_rng();
	rng.gen_range(0..=num)
}

/**
 * Generate an array of captcha characters
 * params num - The number of digits of the verification code and the maximum cannot exceed 53
 */
pub fn get_captcha(num: usize) -> Vec<String> {
	let mut res = vec![];
	for _ in 0..num {
		let rnd = get_rnd(53);
		res.push(BASIC_CHAR[rnd].to_string())
	}
	res
}

/**
 * Get color
 */
pub fn get_color(dark_mode: bool) -> Rgb<u8> {
	let rnd = get_rnd(4);
	if dark_mode {
		return Rgb(DARK_BASIC_COLOR[rnd]);
	}
	Rgb(LIGHT_BASIC_COLOR[rnd])
}

/**
 * Generate random numbers between two numbers
 * params min – minimum
 *        max – maximum value
 * return: random number
 */
pub fn get_next(min: f32, max: u32) -> f32 {
	min + get_rnd(max as usize - min as usize) as f32
}

/**
 * Get font
 */
pub fn get_font() -> Font<'static> {
	let font = Vec::from(include_bytes!("../../fonts/arial.ttf") as &[u8]);
	Font::try_from_vec(font).unwrap()
}

/**
 * Get an image with a white background
 */
pub fn get_image(width: u32, height: u32, dark_mode: bool) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
	ImageBuffer::from_fn(width, height, |_, _| {
		if dark_mode {
			return image::Rgb(DARK);
		}
		image::Rgb(LIGHT)
	})
}

/**
 * Loop to write captcha characters on background image
 * params res    - Array of verification code characters to be written
 *        image  - Background picture
 */
pub fn cyclic_write_character(
	res: &[String],
	image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
	dark_mode: bool,
) {
	let c = (image.width() - 10) / res.len() as u32;
	let y = image.height() / 2 - 20;

	let scale = match res.len() {
		1..=3 => SCALE_LG,
		4..=5 => SCALE_MD,
		_ => SCALE_SM,
	};
	
	for (i, _) in res.iter().enumerate() {
		let text = &res[i];
		
		draw_text_mut(image, get_color(dark_mode), 5 + (i as u32 * c) as i32, y as i32, scale, &get_font(), text);
	}
}

/**
 * Draw interference lines
 * params image  - Background picture
 */
pub fn draw_interference_line(image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, dark_mode: bool) {
	let width = image.width();
	let height = image.height();
	let x1: f32 = 5.0;
	let y1 = get_next(x1, height / 2);
	
	let x2 = (width - 5) as f32;
	let y2 = get_next((height / 2) as f32, height - 5);
	
	let ctrl_x = get_next((width / 4) as f32, width / 4 * 3);
	let ctrl_y = get_next(x1, height - 5);
	
	let ctrl_x2 = get_next((width / 4) as f32, width / 4 * 3);
	let ctrl_y2 = get_next(x1, height - 5);
	// Randomly draw bezier curves
	draw_cubic_bezier_curve_mut(
		image,
		(x1, y1),
		(x2, y2),
		(ctrl_x, ctrl_y),
		(ctrl_x2, ctrl_y2),
		get_color(dark_mode),
	);
}

/**
 * Draw a distraction circle
 * params num    - Number of circles drawn
 *        image  - Background picture
 */
pub fn draw_interference_ellipse(
	num: usize,
	image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
	dark_mode: bool,
) {
	for _ in 0..num {
		let w = (10 + get_rnd(5)) as i32;
		let x = get_rnd((image.width() - 25) as usize) as i32;
		let y = get_rnd((image.height() - 15) as usize) as i32;
		draw_hollow_ellipse_mut(image, (x, y), w, w, get_color(dark_mode));
	}
}

/**
 * Convert image to JPEG base64 string
 * parma image - Image
 */
pub fn to_base64_str(image: &DynamicImage, compression: u8) -> String {
	let mut buf = Cursor::new(Vec::new());
	image.write_to(&mut buf, Jpeg(compression)).unwrap();
	let res_base64 = general_purpose::STANDARD.encode(buf.into_inner());
	format!("data:image/jpeg;base64,{}", res_base64)
}



pub struct Captcha {
	pub text: String,
	pub image: DynamicImage,
	pub compression: u8,
	pub dark_mode: bool,
}

impl Captcha {
	pub fn to_base64(&self) -> String {
		to_base64_str(&self.image, self.compression)
	}
}

#[derive(Default)]
pub struct CaptchaBuilder {
	text: Option<String>,
	width: Option<u32>,
	height: Option<u32>,
	dark_mode: Option<bool>,
	complexity: Option<u32>,
	compression: Option<u8>,
}

impl CaptchaBuilder {
	pub fn new() -> Self {
		CaptchaBuilder {
			text: None,
			width: None,
			height: None,
			dark_mode: None,
			complexity: None,
			compression: Some(40),
		}
	}

	pub fn text(mut self, text: String) -> Self {
		self.text = Some(text);
		self
	}

	pub fn length(mut self, length: usize) -> Self {
		// Generate an array of captcha characters
		let res = captcha::get_captcha(length);
		self.text = Some(res.join(""));
		self
	}

	pub fn width(mut self, width: u32) -> Self {
		self.width = Some(width);
		self
	}

	pub fn height(mut self, height: u32) -> Self {
		self.height = Some(height);
		self
	}

	pub fn dark_mode(mut self, dark_mode: bool) -> Self {
		self.dark_mode = Some(dark_mode);
		self
	}

	pub fn complexity(mut self, complexity: u32) -> Self {
		let mut complexity = complexity;
		if complexity > 10 { complexity = 10; }
		if complexity < 1 { complexity = 1; }
		self.complexity = Some(complexity);
		self
	}

	pub fn compression(mut self, compression: u8) -> Self {
		self.compression = Some(compression);
		self
	}

	pub fn build(self) -> Captcha {
		let text = self.text.unwrap_or(captcha::get_captcha(5).join(""));
		let width = self.width.unwrap_or(130);
		let height = self.height.unwrap_or(40);
		let dark_mode = self.dark_mode.unwrap_or(false);
		let complexity = self.complexity.unwrap_or(1);
		let compression = self.compression.unwrap_or(40);

		// Create a white background image
		let mut image = get_image(width, height, dark_mode);

		let res: Vec<String> = text.chars().map(|x| x.to_string()).collect();

		// Loop to write the verification code string into the background image
		cyclic_write_character(&res, &mut image, dark_mode);

		// Draw interference lines
		draw_interference_line(&mut image, dark_mode);
		draw_interference_line(&mut image, dark_mode);

		// Draw a distraction circle
		draw_interference_ellipse(2, &mut image, dark_mode);
		draw_interference_ellipse(2, &mut image, dark_mode);

		if complexity > 1 {
			gaussian_noise_mut(&mut image, (complexity - 1) as f64, ((5 * complexity) - 5) as f64, ((5 * complexity) - 5) as u64);
			salt_and_pepper_noise_mut(&mut image, (0.002 * complexity as f64) - 0.002, (0.5 * complexity as f64) as u64);
		}

		Captcha {
			text,
			image: DynamicImage::ImageRgb8(image),
			compression,
			dark_mode,
		}
	}
}