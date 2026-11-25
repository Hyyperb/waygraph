use std::ptr::null;

pub struct QuadRenderer {
    shader_program: u32,
    vao: u32,
}

impl QuadRenderer {
    pub fn new() -> Self {
        let shader_program = create_shader_program();

        #[rustfmt::skip]
        let vertices: [f32; _] = [
            0.0, 1.0,
            0.0, 0.0,
            1.0, 0.0,

            1.0, 0.0,
            1.0, 1.0,
            0.0, 1.0
        ];

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
                null(),
            );
            gl::EnableVertexAttribArray(0);

            gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);
        }

        Self {
            shader_program,
            vao,
        }
    }

    pub fn render(&self, tex: u32, x: f32, y: f32, w: f32, h: f32) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, tex);
            gl::BindVertexArray(self.vao);
            gl::UseProgram(self.shader_program);
            gl::Uniform2f(
                gl::GetUniformLocation(self.shader_program, c"quadSize".as_ptr()),
                w,
                h,
            );
            gl::Uniform2f(
                gl::GetUniformLocation(self.shader_program, c"quadPos".as_ptr()),
                x,
                y,
            );
            gl::Uniform2f(
                gl::GetUniformLocation(self.shader_program, c"offset".as_ptr()),
                0.0,
                0.0,
            );
            gl::Uniform2f(
                gl::GetUniformLocation(self.shader_program, c"viewportSize".as_ptr()),
                1920.0,
                1080.0,
            );

            gl::DrawArrays(gl::TRIANGLES, 0, 6);
        }
    }
}

fn create_shader_program() -> u32 {
    // TODO: pass initial viewport size directly into shader compilation
    // and update it on size change only
    unsafe {
        let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);

        const VERT_SHADER: &str = r#"
            #version 330 core

            in vec2 aPos;
            uniform vec2 viewportSize;
            uniform vec2 quadSize;
            uniform vec2 quadPos;
            uniform vec2 offset;
            out vec2 TexCoord;

            vec2 translate(vec2 aPos, vec2 pos, vec2 dim, vec2 offset){
                return ((aPos.xy * dim + pos  - vec2(1 - offset.x, offset.y)) / viewportSize.xy) * vec2(2,-2) + vec2(-1, 1);
            }
            
            void main(){
                vec2 offset = vec2(-1.5, 0.5);
                vec2 transform = vec2(2, -2);
                gl_Position = vec4(translate(aPos.xy, quadPos, quadSize, vec2(0,0)).xy , 0.0, 1.0);
                TexCoord = vec2(aPos.xy);
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
                final_color = texture(glyphTexture, TexCoord);
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
