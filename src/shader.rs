use web_sys::{WebGl2RenderingContext as GL, WebGlProgram};
use std::ops::Not;
use crate::util::log;

#[derive(Clone)]
pub struct Shader {
    id: Option<WebGlProgram>,
    gl: Option<GL>,
    vs_source: String,
    fs_source: String,
}

impl Shader {
    pub fn new(vs_source: &str, fs_source: &str) -> Shader {
        Self {
            id: None,
            gl: None,
            vs_source: vs_source.to_string(),
            fs_source: fs_source.to_string(),
        }
    }

    pub fn compile(&mut self, gl_context: Option<GL>) {
        log(self.vs_source.as_str().to_string());

        let gl = gl_context.as_ref().expect("Context not found");
        let vert_shader = gl.create_shader(GL::VERTEX_SHADER).unwrap();
        gl.shader_source(&vert_shader, self.vs_source.as_str());
        gl.compile_shader(&vert_shader);
        let vert_err = format!("{}", gl.get_shader_info_log(&vert_shader).expect("foobat")).to_string();
        vert_err.is_empty().not().then(|| log(vert_err));

        let frag_shader = gl.create_shader(GL::FRAGMENT_SHADER).unwrap();
        gl.shader_source(&frag_shader, self.fs_source.as_str());
        gl.compile_shader(&frag_shader);
        let frag_err = format!("{}", gl.get_shader_info_log(&frag_shader).expect("foobat")).to_string();
        frag_err.is_empty().not().then(|| log(frag_err));

        let shader_program = gl.create_program().unwrap();
        gl.attach_shader(&shader_program, &vert_shader);
        gl.attach_shader(&shader_program, &frag_shader);
        gl.link_program(&shader_program);

        self.gl = gl_context;
        self.id = Some(shader_program);

    }

    pub fn use_shader(self) {
        let gl = self.gl.expect("Could not get GL context");

        gl.use_program(Some(&self.id.expect("foobared")));
    }

    pub fn shader_id(self) -> Option<WebGlProgram> {
        self.id
    }
}
