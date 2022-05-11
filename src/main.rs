#![allow(unused_imports)]

#[allow(dead_code)]

use std::ops::Not;
use gloo_render::{request_animation_frame, AnimationFrame};
use gloo_console::log;
use gloo_events::{EventListener};
use wasm_bindgen::prelude::*;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext as GL};
use yew::html::Scope;
use yew::{html, Component, Context, Html, NodeRef};
use glam::{Mat4, Vec3};

// #[wasm_bindgen]
// extern "C" {
//     #[wasm_bindgen(js_namespace = console)]
//     fn log(s: &str);
// }

pub enum Msg {
    Render(f64),
    Resize(),
}

pub struct Dimensions {
    width: f64,
    height: f64
}

pub struct App {
    gl: Option<GL>,
    node_ref: NodeRef,
    _render_loop: Option<AnimationFrame>,
    window: web_sys::Window,
    window_dims: Dimensions
}

#[allow(unused_unsafe)]
fn log(message: String) -> bool {
    unsafe { log!(message) }
    true
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        // Do init stuff here
        let window = web_sys::window().expect("Window not available.");

        log(format!("{}", "Initializing... "));        

        // let _on_click = EventListener::new(&window, "resize", move |_event| {
        //     log("message".to_string());
        // });

        Self {
            gl: None,
            node_ref: NodeRef::default(),
            _render_loop: None,
            window: window,
            window_dims: {Dimensions { width: 0.0, height: 0.0 }}
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Render(timestamp) => {
                self.render_gl(timestamp, ctx.link());
                false
            }
            Msg::Resize() => {
                log("Resize was called.".to_string());
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        // Print window dims
        
        // Format dims for initializing the canvas
        // let height: String = self.window.inner_height().expect("error").as_f64().expect("error").to_string();
        // let width: String = self.window.inner_width().expect("error").as_f64().expect("error").to_string();

        log(format!("View -> Width: {}, Height: {}", self.window_dims.width, self.window_dims.height).to_string());

        // let link = _ctx.link().clone();
        // let resize = link.callback(|e: Event| Msg::Resize(e));

        // initialize the canvas
        html! {
            <div>
                <div class="test"><h1>{ "Hello world!" }</h1></div>
                <canvas class="background" ref={self.node_ref.clone()} width={self.window_dims.width.to_string()} height={self.window_dims.height.to_string()} />
                // <canvas class="background" ref={self.node_ref.clone()} width={"1674"} height={"1301"} />
            </div>
        }
   }
    

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        // Post canvas render initialization

        // Set window_dims
        self.window_dims.width = self.window.inner_width().expect("error").as_f64().expect("error");
        self.window_dims.height = self.window.inner_height().expect("error").as_f64().expect("error");
        log(format!("Rendered -> Width: {}, Height: {}", self.window_dims.width, self.window_dims.height).to_string());

        // Get WebGL context
        let canvas = self.node_ref.cast::<HtmlCanvasElement>().unwrap();
        let gl: JsValue = canvas.get_context("webgl2").unwrap().into();
        let gl: GL = gl.into();
        self.gl = Some(gl);
        let gl = self.gl.as_ref().expect("GL Context not initialized!");

        // WebGL initialization
        gl.viewport(0, 0, self.window_dims.width as i32, self.window_dims.height as i32);
        gl.clear_color(0.0, 0.0, 0.0, 1.0);

        // Setup request_animation_frame()
        if first_render {
            // The callback to request animation frame is passed a time value which can be used for
            // rendering motion independent of the framerate which may vary.
            let handle = {
                let link = ctx.link().clone();
                request_animation_frame(move |time| link.send_message(Msg::Render(time)))
            };
            // A reference to the handle must be stored, otherwise it is dropped and the render
            // won't occur.
            self._render_loop = Some(handle);

            // Resize the initial canvas
            ctx.link().send_message(Msg::Resize());
        }
    }
}

impl App {
    fn render_gl(&mut self, timestamp: f64, link: &Scope<Self>) {
        let gl = self.gl.as_ref().expect("GL Context not initialized!");

        // gl.clear_color(0.0, 0.0, 0.0, 1.0);
        gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);


        // Shader program
        let vert_code = include_str!("./basic.vert");
        let frag_code = include_str!("./basic.frag");

        let vert_shader = gl.create_shader(GL::VERTEX_SHADER).unwrap();
        gl.shader_source(&vert_shader, vert_code);
        gl.compile_shader(&vert_shader);
        let vert_err = format!("{}", gl.get_shader_info_log(&vert_shader).expect("foobat")).to_string();
        vert_err.is_empty().not().then(|| log(vert_err));

        let frag_shader = gl.create_shader(GL::FRAGMENT_SHADER).unwrap();
        gl.shader_source(&frag_shader, frag_code);
        gl.compile_shader(&frag_shader);
        let frag_err = format!("{}", gl.get_shader_info_log(&frag_shader).expect("foobat")).to_string();
        frag_err.is_empty().not().then(|| log(frag_err));

        let shader_program = gl.create_program().unwrap();
        gl.attach_shader(&shader_program, &vert_shader);
        gl.attach_shader(&shader_program, &frag_shader);
        gl.link_program(&shader_program);

        gl.use_program(Some(&shader_program));

        let vao = gl.create_vertex_array().unwrap();
        gl.bind_vertex_array(Some(&vao));
        
        // Verts
        let vertices: Vec<f32> = vec![
            // vertices (3) |  colors(3)
            -1.0, -1.0, 0.0, 1.0, 0.0, 0.0,
            -1.0,  1.0, 0.0, 0.0, 1.0, 0.0,
             1.0,  0.0, 0.0, 0.0, 0.0, 1.0,
        ];
        let vertex_buffer = gl.create_buffer().unwrap();
        let verts = js_sys::Float32Array::from(vertices.as_slice());

        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vertex_buffer));
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &verts, GL::STATIC_DRAW);

        // Attach the position vector as an attribute for the GL context.
        let position = 0 as u32;
        gl.vertex_attrib_pointer_with_i32(position, 3, GL::FLOAT, false, 6*4, 0);
        gl.enable_vertex_attrib_array(position);

        let color = 1 as u32;
        gl.vertex_attrib_pointer_with_i32(color, 3, GL::FLOAT, false, 6*4, 3*4);
        gl.enable_vertex_attrib_array(color);

        // log(color.to_string());


        
        // Instance vbo
        const LEN: usize = 16 * 100;
        let mut translations: [f32; LEN] = [0.0; LEN];
        for i in 0..100 {
            // Create modelview matrix for each instance
            let trans_offset: f32 = -50.0 + i as f32;
            
            let time = timestamp as f32 / 300.0;
            let scale = Vec3::new(3.0, 3.0, 3.0);
            let axis  = Vec3::new(1.0, 0.0, 0.0);
            let rotation = glam::Quat::from_axis_angle(axis, time * 0.3);
            let translation = Vec3::new(1.0 * trans_offset, -1.0 * trans_offset, -50.0);
            let modelview: Mat4 = Mat4::from_scale_rotation_translation(scale, rotation, translation);

            // Load it into an array for js consumption
            let matrix = modelview.to_cols_array();
            let offset = i * 16;
            for j in 0..16 {
                translations[offset + j] = matrix[j];
            }
        }

        // let vec: Vec<f32> = translations.iter().fold(vec![], |acc, slice| slice.to_vec().map(|x| acc.push(x)));
        let js_translations = js_sys::Float32Array::from(translations.as_slice());
        // for chunk in js_translations.to_vec().chunks(16) {
        //     log(format!("{:?}", chunk));
        // }
        // log(format!("[3.0, 0.0, 0.0, 0.0, 0.0, 2.901498, 0.76243615, 0.0, 0.0, -0.76243615,------>{:?}", js_translations.to_vec()));
        
        let instance_buffer = gl.create_buffer().unwrap();
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&instance_buffer));
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &js_translations, GL::STATIC_DRAW);

        for i in 0..4 {
            let attr_loc = 2 + i;
            gl.vertex_attrib_pointer_with_i32(attr_loc, 4, GL::FLOAT, false, 4*4*4, (i*4*4) as i32);
            gl.enable_vertex_attrib_array(attr_loc);
            gl.vertex_attrib_divisor(attr_loc, 1);
        }
        

        // attach the time as a uniform for the GL context.
        let time = gl.get_uniform_location(&shader_program, "u_time");
        gl.uniform1f(time.as_ref(), timestamp as f32);

        // Add perspective transform
        let proj: Mat4 = Mat4::perspective_rh_gl(45.0 * 3.14195 / 180.0,
                                                 self.window_dims.width as f32/ self.window_dims.height as f32,
                                                 0.1,
                                                 1000.0);
        let perspective = gl.get_uniform_location(&shader_program, "u_proj");
        gl.uniform_matrix4fv_with_f32_array(perspective.as_ref(), false, proj.as_ref());

        // Add modelview transform
        // let mut rng = rand::thread_rng();
        let time = timestamp as f32 / 200.0;
        let scale = Vec3::new(3.0, 3.0, 3.0);
        let axis  = Vec3::new(1.0, 0.0, 0.0);
        let rotation = glam::Quat::from_axis_angle(axis, timestamp as f32 / 1000.0);
        let translation = Vec3::new((time / 2.0).sin() * 2.0, time % 20.0 - 10.0, -50.0);
        let modelview: Mat4 = Mat4::from_scale_rotation_translation(scale, rotation, translation);
        // log(format!("{}", modelview).to_string());
        let modelview_loc = gl.get_uniform_location(&shader_program, "u_modelview");
        gl.uniform_matrix4fv_with_f32_array(modelview_loc.as_ref(), false, modelview.as_ref());

        // log(format!("-------->{:?}", modelview));
        
        gl.draw_arrays_instanced(GL::TRIANGLES, 0, 3, 100);
        // gl.draw_arrays(GL::TRIANGLES, 0, 3);

        let handle = {
            let link = link.clone();
            request_animation_frame(move |time| link.send_message(Msg::Render(time)))
        };

        // A reference to the new handle must be retained for the next render to run.
        self._render_loop = Some(handle);
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
