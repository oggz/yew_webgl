#![allow(unused_imports)]
#[allow(dead_code)]

use std::borrow::{BorrowMut, Borrow};
use std::ops::Deref;
use std::ops::Not;
use gloo::render::{request_animation_frame, AnimationFrame};
use gloo::events::EventListener;
use gloo::console::log;
use gloo::timers::callback::Timeout;
use mesh::Mesh;
use rand::Rng;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext as GL, Event};
use yew::html::Scope;
use yew::use_state;
use yew::{html, function_component, Component, Context, Html, NodeRef, Callback, MouseEvent, KeyboardEvent, classes};
use glam::*;

mod util;
mod shader;
mod mesh;
mod renderer;
mod camera;
mod components;
use crate::util::log;
use crate::shader::Shader;
// use crate::components::HelloWorld;



pub enum Msg {
    Render(f64),
    Resize(),
    Zoom(bool),
    KeyPressed(KeyboardEvent),
    AddTriangles(),
    RemoveTriangles(),
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
    window_dims: Dimensions,
    mouse_move: EventListener,
    view: Mat4,
    shader: Shader,
    mesh: Mesh,
    tpos: Vec<Mat4>,
}




impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        // Do init stuff here
        let window = web_sys::window().expect("Window not available.");
        let document = window.document().expect("Document not available.");

        log(format!("{}", "Initializing... "));        

        // let _on_click = EventListener::new(&window, "resize", move |_event| {
        //     log("message".to_string());
        // });


        Self {
            gl: None,
            node_ref: NodeRef::default(),
            _render_loop: None,
            window,
            window_dims: {Dimensions { width: 0.0, height: 0.0 }},
            mouse_move: EventListener::new(&document, "keypress", move |_event| {
                log("message".to_string());
            }),
            view: Mat4::look_at_rh(Vec3::new(0.0, 0.0, 75.0),
                                   Vec3::new(0.0, 0.0, 0.0),
                                   Vec3::new(0.0, 1.0, 75.0)),
            shader: Shader::new(include_str!("./basic.vert"), include_str!("./basic.frag")),
            mesh: Mesh::new(vec![], vec![]),
            tpos: vec![],
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Render(timestamp) => {
                self.render_gl(timestamp, ctx.link());
                false
            }
            Msg::Resize() => {
                self.window_dims.width = self.window.outer_width().expect("error").as_f64().expect("error");
                self.window_dims.height = self.window.outer_height().expect("error").as_f64().expect("error");
                log("Resize was called.".to_string());
                true
            }
            Msg::Zoom(dir) => {
                let zoom_amount = 200.0;
                match dir {
                    true => {
                        let (s, r, t) = self.view.to_scale_rotation_translation();
                        let t = Vec3::new(t.x, t.y, t.z + zoom_amount);
                        self.view = Mat4::from_scale_rotation_translation(s, r, t);
                        return false;
                    },
                    false => {
                        let (s, r, t) = self.view.to_scale_rotation_translation();
                        let t = Vec3::new(t.x, t.y, t.z - zoom_amount);
                        self.view = Mat4::from_scale_rotation_translation(s, r, t);
                        return false;
                    }
                    _ => ()
                }
                
                true
            }
            Msg::KeyPressed(e) => {
                log("KeyPressed was called.".to_string());

                match e.key().as_str() {
                    "w" => {
                        let (s, r, t) = self.view.to_scale_rotation_translation();
                        let t = Vec3::new(t.x, t.y, t.z + 50.0);
                        self.view = Mat4::from_scale_rotation_translation(s, r, t);
                        return false;
                    },
                    "s" => {
                        let (s, r, t) = self.view.to_scale_rotation_translation();
                        let t = Vec3::new(t.x, t.y, t.z - 50.0);
                        self.view = Mat4::from_scale_rotation_translation(s, r, t);
                        return false;
                    }
                    _ => {return false}
                }
            }
            Msg::AddTriangles() => {
                App::add_triangles(self);
                true
            }
            Msg::RemoveTriangles() => {
                if !(self.tpos.len() <= 0) {
                    self.tpos.drain(self.tpos.len()-300..);
                    true
                }
                else {
                    true
                }
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // Print window dims
        log(format!("View -> Width: {}, Height: {}", self.window_dims.width, self.window_dims.height).to_string());

        // Window dims from window after initial render.
        let width = self.window_dims.width.to_string();
        let height = self.window_dims.height.to_string();
        let tcount = self.tpos.len().to_string();

        // initialize the canvas
        html! {
        <div
            onkeypress={ctx.link().callback(|_| Msg::Resize())}>
          <div class="container">
            <div class={classes!( "radius-container")}>
              <p> {"Personal website is down for maintenance. In the mean time please enjoy this demo! It utilizes Rust copmiled to web assembly(wasm) in order to allow greater performance than js/ts. Use the buttons below to add/remmove triangles and to zoom in/out. Zoom out to break the illusion."} </p>
              <p> {"Code is not yet optimized and triangle postion update math is inefficient and single threaded so more traingles will slow down rendering rather quickly on older hardware, however your browser should remain responsive as it implements request_animation_frame. Code is available on my "}<a href="http://github.com/oggz/yew_webgl">{"github."}</a> </p>
              <p> {format!("{}: {}", "Triangle count", {tcount})} </p>
              <button onclick={ctx.link().callback(|_| Msg::AddTriangles())}>{"Moar triangles"}</button>
              <button onclick={ctx.link().callback(|_| Msg::RemoveTriangles())}>{"Less triangles"}</button>
              <button onclick={ctx.link().callback(|_| Msg::Zoom(true))}>{"Zoom in"}</button>
              <button onclick={ctx.link().callback(|_| Msg::Zoom(false))}>{"Zoom out"}</button>
            </div>
          </div>
          <canvas id="bg-canvas" class="background" ref={self.node_ref.clone()} width={width} height={height}/>
        </div>
        }
   }
    

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        let link = ctx.link().clone();

            // set window_dims
            log(format!("Rendered -> Width: {}, Height: {}", self.window_dims.width, self.window_dims.height + 100.0).to_string());

            // Get WebGL context
            let canvas = self.node_ref.cast::<HtmlCanvasElement>().unwrap();
            let gl: JsValue = canvas.get_context("webgl2").unwrap().into();
            let gl: GL = gl.into();
            self.gl = Some(gl);
            let gl = self.gl.as_ref().expect("GL Context not initialized!");

            // WebGL initialization
            gl.viewport(0, 0, self.window_dims.width as i32, self.window_dims.height as i32);
            gl.clear_color(0.0, 0.0, 0.0, 1.0);
            gl.enable(GL::DEPTH_TEST);

            gl.enable(GL::BLEND);
            gl.blend_func(GL::ONE, GL::ONE_MINUS_SRC_ALPHA);

        
        // Setup request_animation_frame()
        if first_render {
            log(format!("{:?}", "first render"));

            // let document  = self.window.document().expect("Document not available");
            // let element = document.get_element_by_id("bg-canvas").expect("Element not available");
            self.mouse_move = EventListener::new(&self.window.window(), "resize", move |_event| {
                log("resized".to_string());
                    link.send_message(Msg::Resize());
            });
            // self.mouse_move = EventListener::new(&document, "keydown", move |_event| {
            //     log("message".to_string());
            //     if let Some(e) = _event.dyn_ref::<KeyboardEvent>() {
            //         log(e.key_code().to_string());
            //         link.send_message(Msg::KeyPressed(e.clone()));
            //     }
            // });
            
            // log(format!("{:?}", _on_mouse_move.target()));

            // Compile Shader
            self.shader.compile(self.gl.clone());

            // Send mesh data to GPU
            self.mesh.bind_buffers(self.gl.as_ref());


            App::add_triangles(self);
            // The callback to request animation frame is passed a time value which can be used for
            // rendering motion independent of the framerate which may vary.
            let handle = {
                let link = ctx.link().clone();
                request_animation_frame(move |time| link.send_message(Msg::Render(time)))
            };
            self._render_loop = Some(handle); // Must store handle to prevent free.

            // Resize the initial canvas
            ctx.link().send_message(Msg::Resize());
        }
    }
}

impl App {
    fn add_triangles(&mut self) {
        let mut rng = rand::thread_rng();
        for i in 0..300 {

            let scale = Vec3::new(10.0 * rng.gen::<f32>(),
                                  10.0 * rng.gen::<f32>(),
                                  10.0 * rng.gen::<f32>());
            let axis  = Vec3::new(1.0 * rng.gen::<f32>(),
                                  1.0 * rng.gen::<f32>(),
                                  1.0 * rng.gen::<f32>());
            let rotation = Quat::from_axis_angle(axis, rng.gen::<f32>());
            let translation = Vec3::new((rng.gen::<f32>() - 0.5) * 600.0,
                                        (rng.gen::<f32>() - 0.5) * 600.0,
                                        (rng.gen::<f32>() - 1.0) * 600.0);
            let modelview: Mat4 = Mat4::from_scale_rotation_translation(scale, rotation, translation);

            self.tpos.push(modelview);
        }
    }
    
    fn render_gl(&mut self, timestamp: f64, link: &Scope<Self>) {
        let gl = self.gl.as_ref().expect("GL Context not initialized!");

        // Clear framebuffer
        gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

        // Shader program
        self.shader.clone().use_shader();


        // Update instance VBO data (this is horribly inefficient. It's been a while since i've done any linear algebra.
        // Should store translation, rotation, scale for each instance instead of Mat4 modelview.
        let mut tpos = vec![];
        // let mut rng = rand::thread_rng();
        for i in 0..self.tpos.len() {
            // Create modelview matrix for each instance
            // let trans_offset: f32 = -50.0 + i as f32;
            
            // let time = timestamp as f32 / 300.0;
            // let scale = Vec3::new(2.0, 2.0, 2.0);
            // let axis  = Vec3::new(1.0, 0.0, 0.0);
            // let rotation = glam::Quat::from_axis_angle(axis, time * 0.3 * trans_offset);
            // let translation = Vec3::new(1.0 * trans_offset + (time * 0.3).sin(), -1.0 * trans_offset, 0.0);
            // let modelview: Mat4 = Mat4::from_scale_rotation_translation(scale, rotation, translation);

            let axis  = Vec3::new(1.0, 0.0, 1.0);
            let rotation = Mat4::from_quat(Quat::from_axis_angle(axis, 0.05));
            let translation = Mat4::from_translation(Vec3::new(0.0, -0.5, 0.0));
            // let new_modelview = Mat4::from_rotation_translation(rotation, translation);
            
            if self.tpos[i].to_scale_rotation_translation().2.y < -300.0 {
                // log(format!("{:?}", self.tpos[i].to_scale_rotation_translation().2.y()));
                self.tpos[i] = Mat4::from_translation(Vec3::new(0.0, 600.0, 0.0)) * self.tpos[i];
            }

            let modelview: Mat4 = translation * self.tpos[i] * rotation;

            self.tpos[i] = modelview;
            for i in modelview.to_cols_array() {
                tpos.push(i);
            }
        }

        // let mut tpos = vec![];
        // self.tpos = self.tpos.iter()
        //     .map(|m| {
        //         m.mul_mat4(&Mat4::from_translation(Vec3::new(0.0, -0.01, 0.0)))
        //             .mul_mat4(other)
        //     }).collect();
        // for modelview in self.tpos.clone() {
        //     for i in modelview.to_cols_array() {
        //         tpos.push(i);
        //     }
        // }
        
        let mesh = self.mesh.borrow_mut();
        mesh.set_instance_data(self.gl.as_ref(), tpos.as_slice());
        


        // Set shader uniforms
        let shader_program = &self.shader.clone().shader_id().unwrap();

        // Attach the time as a uniform for the GL context.
        let u_time = gl.get_uniform_location(shader_program, "u_time");
        gl.uniform1f(u_time.as_ref(), timestamp as f32);

        // Attach the view matrix.
        // let mut rng = rand::thread_rng();
        let time = timestamp * 0.003;
        let u_view = gl.get_uniform_location(shader_program, "u_view");
        let rotation = Mat4::from_rotation_z(0.0);
        let translation = Mat4::from_translation(Vec3::new(0.0, 0.0, -50.0));
        // let rotation = Mat4::from_rotation_z(timestamp as f32 * 0.001);
        // let translation = Mat4::from_translation(Vec3::new(0.0, 0.0, time.sin() as f32 * 10.0));
        let view = self.view * rotation * translation;
        gl.uniform_matrix4fv_with_f32_array(u_view.as_ref(), false, view.as_ref());
        
        // Attach the proj matrix.
        let proj: Mat4 = Mat4::perspective_rh_gl(45.0 * 3.14195 / 180.0,
                                                 self.window_dims.width as f32/ self.window_dims.height as f32,
                                                 0.1,
                                                 5000.0);
        let u_proj = gl.get_uniform_location(shader_program, "u_proj");
        gl.uniform_matrix4fv_with_f32_array(u_proj.as_ref(), false, proj.as_ref());


        
        // Draw geometry
        self.mesh.clone().draw(self.gl.as_ref());
        // gl.draw_arrays_instanced(GL::TRIANGLES, 0, 3, 100);


        
        let handle = {
            let link = link.clone();
            request_animation_frame(move |time| link.send_message(Msg::Render(time)))
        };

        // A reference to the new handle must be retained for the next render to run.
        self._render_loop = Some(handle);
    }
}

fn main() {
    yew::start_app::<App>();
}
