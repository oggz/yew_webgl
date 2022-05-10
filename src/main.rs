#![allow(unused_imports)]
#[allow(dead_code)]

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

pub struct App {
    gl: Option<GL>,
    node_ref: NodeRef,
    _render_loop: Option<AnimationFrame>,
    window: web_sys::Window,
    window_dims: (f64, f64),
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

        log(format!("{}", "fppnasad"));        

        let _on_click = EventListener::new(&window, "resize", move |_event| {
            log("message".to_string());
        });

        Self {
            gl: None,
            node_ref: NodeRef::default(),
            _render_loop: None,
            window: window,
            window_dims: (0.0, 0.0),
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

        log(format!("View -> Width: {}, Height: {}", self.window_dims.0, self.window_dims.1).to_string());

        // let link = _ctx.link().clone();
        // let resize = link.callback(|e: Event| Msg::Resize(e));

        // initialize the canvas
        html! {
            <div>
                <div class="test"><h1>{ "Hello world!" }</h1></div>
                <canvas class="background" ref={self.node_ref.clone()} width={self.window_dims.0.to_string()} height={self.window_dims.1.to_string()} />
                // <canvas class="background" ref={self.node_ref.clone()} width={"1674"} height={"1301"} />
            </div>
        }
   }
    

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        // Once rendered, store references for the canvas and GL context. These can be used for
        // resizing the rendering area when the window or canvas element are resized, as well as
        // for making GL calls.

        self.window_dims = (self.window.inner_width().expect("error").as_f64().expect("error"),
                            self.window.inner_height().expect("error").as_f64().expect("error"));

        log(format!("Rendered -> Width: {}, Height: {}", self.window_dims.0, self.window_dims.1).to_string());

        let canvas = self.node_ref.cast::<HtmlCanvasElement>().unwrap();

        let gl: JsValue = canvas.get_context("webgl2").unwrap().into();
        let gl: GL = gl.into();

        self.gl = Some(gl);
        let gl = self.gl.as_ref().expect("GL Context not initialized!");

        // In a more complex use-case, there will be additional WebGL initialization that should be
        // done here, such as enabling or disabling depth testing, depth functions, face
        // culling etc.

        gl.viewport(0, 0, self.window_dims.0 as i32, self.window_dims.1 as i32);
        gl.clear_color(0.0, 0.0, 0.0, 1.0);

        if first_render {
            // The callback to request animation frame is passed a time value which can be used for
            // rendering motion independent of the framerate which may vary.
            let handle = {
                let link = ctx.link().clone();
                request_animation_frame(move |time| link.send_message(Msg::Render(time)))
            };

            ctx.link().send_message(Msg::Resize());

            // A reference to the handle must be stored, otherwise it is dropped and the render
            // won't occur.
            self._render_loop = Some(handle);
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

        let frag_shader = gl.create_shader(GL::FRAGMENT_SHADER).unwrap();
        gl.shader_source(&frag_shader, frag_code);
        gl.compile_shader(&frag_shader);

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
        let position = gl.get_attrib_location(&shader_program, "a_position") as u32;
        gl.vertex_attrib_pointer_with_i32(position, 3, GL::FLOAT, false, 6*4, 0);
        gl.enable_vertex_attrib_array(position);

        let color = gl.get_attrib_location(&shader_program, "a_color") as u32;
        gl.vertex_attrib_pointer_with_i32(color, 3, GL::FLOAT, false, 6*4, 3*4);
        gl.enable_vertex_attrib_array(color);


        
        // Instance vbo
        let mut translations: Vec<f32> = Vec::with_capacity(100);
        for i in 0..300 {
            // translations.push(Mat4::from_translation(Vec3::new(0.0, 0.0, 1.0 * i as f32)));
            translations.push(i as f32);
        }

        let js_translations = js_sys::Float32Array::from(translations.as_slice());
        
        let instance_buffer = gl.create_buffer().unwrap();
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&instance_buffer));
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &js_translations, GL::STATIC_DRAW);

        let translation_attr = gl.get_attrib_location(&shader_program, "a_translation") as u32;
        gl.vertex_attrib_pointer_with_i32(translation_attr, 3, GL::FLOAT, false, 3*4, 0);
        gl.enable_vertex_attrib_array(translation_attr);
        gl.vertex_attrib_divisor(translation_attr, 1);

        

        // attach the time as a uniform for the GL context.
        let time = gl.get_uniform_location(&shader_program, "u_time");
        gl.uniform1f(time.as_ref(), timestamp as f32);

        // Add perspective transform
        let proj: Mat4 = Mat4::perspective_rh_gl(45.0 * 3.14195 / 180.0,
                                                 self.window_dims.0 as f32/ self.window_dims.1 as f32,
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
