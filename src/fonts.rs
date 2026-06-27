use std::{collections::HashMap, path::PathBuf, ptr::null};

use freetype::face::LoadFlag;

pub struct GlyphRenderCache {
    texture: u32,
    height: f32,
    width: f32,
    bearing_x: f32,
    bearing_y: f32,
}

pub struct FontRenderer {
    font_name: String,
    font_size: u8,
    font_path: String,
    library: freetype::Library,
    face: freetype::Face,
    glyphs: HashMap<char, GlyphRenderCache>,
    shader_program: u32,
    vao: u32,
}
pub fn get_system_fonts(path: Option<String>) -> Vec<PathBuf> {
    let fonts_dir = std::fs::read_dir(path.unwrap_or("/usr/share/fonts/".to_string())).unwrap();

    let mut fonts: Vec<PathBuf> = vec![];

    for file in fonts_dir {
        let file = file.unwrap();
        if file.path().is_dir() {
            fonts.extend(get_system_fonts(
                file.path().to_str().map(|s| s.to_string()),
            ));
        } else {
            fonts.push(file.path());
        }
    }

    fonts
}

pub fn find_font_path(query: &String) -> Option<PathBuf> {
    let fonts = get_system_fonts(None);
    fonts.into_iter().find(|f| {
        f.ends_with(query)
            || f.file_stem()
                .and_then(|s| s.to_str())
                .is_some_and(|s| s == query)
    })
}

impl FontRenderer {
    pub fn new(font_name: String, font_size: u8) -> FontRenderer {
        let font_path = find_font_path(&font_name).unwrap().display().to_string();

        let library = freetype::library::Library::init().unwrap();
        let face = library.new_face(&font_path, 0).unwrap();

        face.set_pixel_sizes(0, font_size as u32).unwrap();

        let mut glyphs = HashMap::new();

        let shader_program = create_shader_program();

        let vertices: [f32; _] = [0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0];
        let mut vao: u32 = 0;
        let mut vbo: u32 = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                std::mem::size_of_val(&vertices) as isize,
                vertices.as_ptr().cast(),
                gl::STATIC_DRAW,
            );
            gl::VertexAttribPointer(
                0,
                2,
                gl::FLOAT,
                gl::FALSE,
                (2 * std::mem::size_of::<f32>()) as i32,
                std::ptr::null(),
            );
            gl::EnableVertexAttribArray(0);

            gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);
        }

        for charcode in 32..128 {
            face.load_char(charcode, LoadFlag::RENDER).unwrap();
            let glyph = face.glyph();
            let bitmap = glyph.bitmap();
            let advance = glyph.advance();

            let mut texture: u32 = 0;
            unsafe {
                gl::GenTextures(1, &mut texture);
                gl::BindTexture(gl::TEXTURE_2D, texture);
            }

            let glyphcache = GlyphRenderCache {
                texture,
                width: bitmap.width() as f32,
                height: bitmap.rows() as f32,
                bearing_x: glyph.bitmap_left() as f32,
                bearing_y: glyph.bitmap_top() as f32,
            };

            unsafe {
                gl::TexImage2D(
                    gl::TEXTURE_2D,
                    0,
                    gl::R8 as i32,
                    glyphcache.width as i32,
                    glyphcache.height as i32,
                    0,
                    gl::RED,
                    gl::UNSIGNED_BYTE,
                    bitmap.buffer().as_ptr().cast(),
                );

                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            }

            glyphs.insert(char::from_u32(charcode as u32).unwrap(), glyphcache);
        }

        FontRenderer {
            font_name,
            font_size,
            font_path,

            library,
            face,
            glyphs,
            shader_program,
            vao,
        }
    }

    pub fn render_text(&self, text: String, x: f64, y: f64) {
        for ch in text.chars() {
            let g = self.glyphs.get(&ch).unwrap();
            unsafe {
                gl::ActiveTexture(gl::TEXTURE0);
                gl::BindTexture(gl::TEXTURE_2D, g.texture);
                gl::UseProgram(self.shader_program);
                gl::DrawArrays(gl::TRIANGLES, 0, 6);
            }
        }
    }
}

fn create_shader_program() -> u32 {
    unsafe {
        let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);

        const VERT_SHADER: &str = r#"
            #version 330 core

            in vec2 charPos;
            out vec2 TexCoord;

            void main(){
                gl_Position = vec4(charPos.x - 0.5, - (charPos.y - 0.5), 0.0, 1.0);
                TexCoord = vec2(charPos.xy);
            }
            "#;

        let raw_source = std::ffi::CString::new(VERT_SHADER).unwrap();
        gl::ShaderSource(vertex_shader, 1, &raw_source.as_ptr(), null());

        gl::CompileShader(vertex_shader);

        let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        assert_ne!(vertex_shader, 0);

        let mut success = 0;
        gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);

        if success != 1 {
            let mut info_log: [i8; 512] = [0; 512];
            gl::GetShaderInfoLog(vertex_shader, 512, &mut (0), info_log.as_mut_ptr());
            let log_message = std::ffi::CStr::from_ptr(info_log.as_mut_ptr());
            println!("SHADER COMPILATION FAILED: {:?}", log_message);
        }

        const FRAG_SHADER: &str = r#"
            #version 330 core
            in vec2 TexCoord;
            uniform sampler2D glyphTexture;
            out vec4 final_color;


            void main() {
                float alpha = texture(glyphTexture, TexCoord).r;
                final_color = vec4(1.0, 1.0, 1.0, alpha);
            }
            "#;

        let raw_source = std::ffi::CString::new(FRAG_SHADER).unwrap();
        gl::ShaderSource(fragment_shader, 1, &raw_source.as_ptr(), null());

        gl::CompileShader(fragment_shader);

        gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);

        if success != 1 {
            let mut info_log: [i8; 512] = [0; 512];
            gl::GetShaderInfoLog(fragment_shader, 512, &mut (0), info_log.as_mut_ptr());
            let log_message = std::ffi::CStr::from_ptr(info_log.as_mut_ptr());
            println!("SHADER COMPILATION FAILED: {:?}", log_message);
        }

        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

        let shader_program = gl::CreateProgram();
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);

        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        shader_program
    }
}

impl std::fmt::Debug for FontRenderer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FontRenderer")
            .field("font_name", &self.font_name)
            .field("font_size", &self.font_size)
            .field("font_path", &self.font_path)
            .field("library", &self.library.raw())
            .field("face", &self.face)
            .field("glyphs cached", &self.glyphs.len())
            .finish()
    }
}
