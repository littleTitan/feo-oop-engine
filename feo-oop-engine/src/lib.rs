//! [![github]](https://github.com/littleTitan/feo-oop-engine)&ensp;[![crates-io]](https://crates.io/crates/feo-oop-engine)&ensp;[![docs-rs]](crate)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K
//!
//! Feo OOP Engine is an object oriented game engine.
//! > procedural macros: [feo-oop-engine-proc-macros](https://docs.rs/feo-oop-engine-proc-macros)
//! ## Compatibility
//!
//! |  OS     | Compatible |
//! | :-----: | :--------: |
//! | Windows | Issue id:1 |
//! | Linux   | Yes        |
//! | OSX     | Yes        |
//!
//! See issue [#1](/../../issues/1) for Windows
//! ## Description
//! The FeO OOP engine is a library I created to help me learn about 3D engines. This 
//! project is composed of two parts. [feo-math](https://github.com/littleTitan/feo-math), 
//! the math boilerplate; and the feo-oop-engine. The engine is built on the 
//! [vulkano](https://vulkano.rs) framework. This program is designed to facilitate 3D 
//! application development. Please note however that this program has its own unique 
//! workflow. Features of the engine include [scripts](###scripts), object oriented 
//! programming (or OOP for short), textures, materials, lights, game objects, and obj
//! and mtl processing.
//! 
//! ## Example
//! 
//! ### Building the Scene
//! 
//! First create a new scene.
//! ```no_run
//! use feo_oop_engine::scene::Scene;
//! 
//! let scene = Scene::new(None);
//! ```
//! This is where all of your game-objects will directly or indirectly exist on. 
//!
//! ### Initialize the Engine With the Scene
//! To create an engine use the `FeoEngine::init(scene, specify_hardware)`. This will create a feo_engine object.
//! ```no_run
//! use feo_oop_engine::FeoEngine;
//! # let scene = feo_oop_engine::scene::Scene::new(None);
//! let mut engine = FeoEngine::init(scene, Some(1));
//! ```
//!
//! ### Build Objects
//! To build objects use the `::new()` constructor for the object you wish to build. You might want to build a light and a camera to be able to see the scene.
//! ```no_run
//! use feo_oop_engine::scene::game_object::obj::Obj;
//! # let scene = feo_oop_engine::scene::Scene::new(None);
//! # let mut engine = feo_oop_engine::FeoEngine::init(scene, Some(1));
//! let obj = Obj::from_obj(
//!    Some("cube"), 
//!    "assets/standard-assets/models/shapes/cube.obj",
//!    None,
//!    None,
//!    None,
//!    None,
//!    true,
//!    engine.globals.clone(),
//!    None
//! );
//! ```
//!
//! ### Pass Objects to Scene
//! Use the `add_child()` function to add the object you created to the scene within the engine.
//! ```no_run
//! use feo_oop_engine::registration::relation::Parent;
//! # let scene = feo_oop_engine::scene::Scene::new(None);
//! # let mut engine = feo_oop_engine::FeoEngine::init(scene, Some(1));
//! # let obj = feo_oop_engine::scene::game_object::obj::Obj::from_obj( Some("cube"), 
//! #    "assets/standard-assets/models/shapes/cube.obj", None, None, None, None,
//! #    true, engine.globals.clone(), None );
//! engine.scene.write().unwrap().add_child(obj.unwrap());
//! ```
//!
//! ### Running the Engine
//! When all the game_objects have been created you can use the run() function of feo_engine to start the engine.
//! ```ignore
//! engine.run()
//! ```
#[macro_use] extern crate lazy_static;

#[macro_use] extern crate feo_oop_engine_proc_macros;

pub mod scene;
pub mod components;
pub mod scripting;
pub mod event;
pub mod graphics;
pub mod registration;

pub mod shaders;
pub mod macros;

pub(crate) mod term_ui;

use {
    self::{
        graphics::frame_system::FrameSystem,
        event::UserEvent,
        scripting::globals::EngineGlobals,
        scene::Scene,
        components::texture::Texture,
        registration::id::IDSystem
    },
    std::{
        sync::{
            Arc,
            RwLock,
        },
        any::Any,
    },
    vulkano::{
        device::{
            Device, 
            DeviceExtensions, 
            Queue
        },
        image::{
            view::ImageView,
            ImageUsage
        }, 
        instance::{
            Instance,
        },
        swapchain::{
            self, 
            AcquireError, 
            ColorSpace, 
            FullscreenExclusive, 
            PresentMode, 
            Surface, 
            SurfaceTransform, 
            Swapchain, 
            SwapchainCreationError
        }, 
        sync::{
            self, 
            FlushError, 
            GpuFuture
        }
    },
    vulkano_win::VkSurfaceBuild,
    winit::{
        dpi::PhysicalSize, 
        event::{
            Event, 
            WindowEvent
        }, 
        event_loop::{
            ControlFlow, 
            EventLoop
        },
        platform::run_return::EventLoopExtRunReturn, 
        window::{
            Window, 
            WindowBuilder
        }
    }
};

/// The Engine
pub struct FeoEngine {
    event_loop: EventLoop<UserEvent<Arc<dyn Any + 'static + Send + Sync>>>,
    surface: Arc<Surface<Window>>,
    queue: Arc<Queue>,

    pub scene: Arc<RwLock<Scene>>,

    pub id_system: IDSystem,

    pub globals: EngineGlobals
}

impl FeoEngine {
    /// Initialize a FeoEngine.
    /// 
    /// # Arguments
    /// * `scene` - The scene in which GameObjects exist.
    /// * `index` - The optional device id. The default (None) displays available devices and prompts for the selection of one. 
    /// # Examples
    /// ```no_run
    /// # use feo_oop_engine::FeoEngine;
    /// # let scene = feo_oop_engine::scene::Scene::new(None);
    /// FeoEngine::init(scene, Some(1)); // Uses the device with id of 1
    /// ```
    pub fn init(scene: Arc<RwLock<Scene>>, index: Option<usize>) -> FeoEngine {
        // Vulkano Instance
        let instance = Instance::new(None, &vulkano_win::required_extensions(), None).unwrap();

        // Physical Device
        let physical = term_ui::prompt_physical_device(&instance, index);

        // event loop
        let event_loop = EventLoop::<UserEvent<Arc<dyn Any + Send + Sync>>>::with_user_event();

        // surface
        let surface = {
            let mut builder = WindowBuilder::new();
            builder.window.inner_size = Some(PhysicalSize::new(1024_u32, 512_u32).into());
            builder.build_vk_surface(&event_loop, instance.clone()).unwrap()
        };

        // get access to the device and get graphics queue
        let (_device, queue) = {
            let queue_family = physical
                .queue_families()
                .find(|&q| q.supports_graphics() && surface.is_supported(q).unwrap_or(false))
                .unwrap();

            let device_ext = DeviceExtensions {
                khr_swapchain: true,
                khr_storage_buffer_storage_class: true,
                ..DeviceExtensions::none()
            };

            let features = physical.supported_features();
            // TODO assertions
            
            let (device, mut queues) = Device::new(
                physical,
                features,
                &device_ext,
                [(queue_family, 0.5)].iter().cloned(),
            ).unwrap();

            (device, queues.next().unwrap())
        };

        Texture::default(queue.clone());

        let id_system = IDSystem::default();
        
        FeoEngine {
            globals: EngineGlobals{ // todo fix
                queue: queue.clone(),
                surface: surface.clone(),
                scene: scene.clone(),
                event_loop_proxy: Arc::new(futures::lock::Mutex::new(event_loop.create_proxy())),
                id_system: id_system.clone()
            },

            //instance,
            event_loop,
            surface,
            queue,

            scene,

            id_system,
        }
    }

    /// Allows the engine to commence excecution.
    pub fn run(&mut self) {
        // get swapchain and images
        let dimensions: [u32; 2] = self.surface.window().inner_size().into();
        let (mut swapchain, _) = {
            let caps = self.surface.capabilities(self.queue.device().physical_device()).unwrap();
            let format = caps.supported_formats[0].0;
            let alpha = caps.supported_composite_alpha.iter().next().unwrap();

            let (swapchain, images) = Swapchain::new(
                self.queue.device().clone(),
                self.surface.clone(),
                caps.min_image_count,
                format,
                dimensions,
                1,
                ImageUsage::color_attachment(),
                &self.queue,
                SurfaceTransform::Identity,
                alpha,
                PresentMode::Fifo,
                FullscreenExclusive::Default,
                true,
                ColorSpace::SrgbNonLinear,
            ).unwrap();

            let images = images
                .into_iter()
                .map(|image| ImageView::new(image).unwrap())
                .collect::<Vec<_>>();
            
            (swapchain, images)
        };

        // Deferred system
        let mut frame_system = FrameSystem::new(self.queue.clone(), swapchain.format(), dimensions);
        
        // Frame Future
        let mut previous_frame_end = Some(sync::now(self.queue.device().clone()).boxed());

        // Event Loop proxy
        let proxy = self.event_loop.create_proxy();
        
        // Self pointer
        let local_self: *mut Self = self;

        self.event_loop.run_return(move | mut event, _, control_flow| {
            // a mutable reference to self
            let local_self = unsafe {&mut *local_self};
    
            // Deal with Event Redundancy
            while let Event::UserEvent(UserEvent::WinitEvent(inner_event)) = event {
                event = match inner_event {
                    Event::UserEvent(boxed_user_event) => Event::UserEvent(*boxed_user_event),
                    Event::NewEvents(start_case) => Event::NewEvents(start_case),
                    Event::WindowEvent { window_id, event } => Event::WindowEvent { window_id, event },
                    Event::DeviceEvent { device_id, event } => Event::DeviceEvent { device_id, event },
                    Event::Suspended => Event::Suspended,
                    Event::Resumed => Event::Resumed,
                    Event::MainEventsCleared => Event::MainEventsCleared,
                    Event::RedrawRequested(window_id) => Event::RedrawRequested(window_id),
                    Event::RedrawEventsCleared => Event::RedrawEventsCleared,
                    Event::LoopDestroyed => Event::LoopDestroyed,
                };
            }

            // Static Event
            let event: Event<'static, UserEvent<Arc<dyn Any + Send + Sync>>> = event.to_static().unwrap();
            
            // Executor for Object event handlers
            let h_executor = {
                let (executor, spawner) = scripting::new_executor_and_spawner(local_self.globals.clone());
                local_self.scene.read().unwrap().spawn_script_handlers(spawner, event.clone());
                executor
            };
            
            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    *control_flow = ControlFlow::Exit;
                },
                Event::UserEvent( UserEvent::RebuildSwapchain ) | 
                Event::WindowEvent {
                    event: WindowEvent::Resized(_),
                    ..
                } => { 
                    // resizing is slower however because no dynamic viewports are used rendering is faster
                    let dimensions: [u32; 2] = local_self.surface.window().inner_size().into();

                    let (new_swapchain, new_images) = 
                        match swapchain.recreate_with_dimensions(dimensions) {
                            Ok(r) => r,
                            Err(SwapchainCreationError::UnsupportedDimensions) => return,
                            Err(e) => panic!("Failed to recreate swapchain: {:?}", e),
                        };

                    let new_images = new_images
                        .into_iter()
                        .map(|image| ImageView::new(image).unwrap())
                        .collect::<Vec<_>>();
                        
                    swapchain = new_swapchain;

                    frame_system.rebuild_dims(&new_images[..]);
                },
                Event::RedrawEventsCleared => {
                    // Clear buffer pool
                    previous_frame_end.as_mut().unwrap().cleanup_finished();

                    // Generate Executor and Spawner for scripts
                    let (executor, spawner) = scripting::new_executor_and_spawner(local_self.globals.clone());
                    local_self.scene.read().unwrap().spawn_script_cores(spawner);

                    // Get the next image
                    let (image_num, suboptimal, acquire_future) =
                        match swapchain::acquire_next_image(swapchain.clone(), None) {
                            Ok(r) => r,
                            Err(AcquireError::OutOfDate) => {
                                proxy.send_event(UserEvent::RebuildSwapchain).unwrap();
                                return;
                            }
                            Err(e) => panic!("Failed to acquire next image: {:?}", e),
                        };
                    
                    // rebuild swapchain if suboptimal
                    if suboptimal { proxy.send_event(UserEvent::RebuildSwapchain).unwrap(); }
                    
                    // Run scripts to completion
                    executor.run(local_self.scene.clone()); // TODO: merge future with other future and start scripts mvd
                    
                    let future = local_self.scene.read().unwrap()
                        .render(local_self.scene.clone(), &mut frame_system, image_num, acquire_future, &mut previous_frame_end)
                        .then_swapchain_present(local_self.queue.clone(), swapchain.clone(), image_num)
                        .then_signal_fence_and_flush();

                    match future {
                        Ok(future) => {
                            previous_frame_end = Some(future.boxed());
                        },
                        Err(FlushError::OutOfDate) => {
                            proxy.send_event(UserEvent::RebuildSwapchain).unwrap();
                            previous_frame_end = Some(sync::now(local_self.queue.device().clone()).boxed());
                        },
                        Err(e) => {
                            println!("Failed to flush future: {:?}", e);
                            previous_frame_end = Some(sync::now(local_self.queue.device().clone()).boxed());
                        }
                    }
                    
                },
                _ => {},
            }
            
            // Force event handlers to Completion
            h_executor.run(local_self.scene.clone());
        });
    }
}
