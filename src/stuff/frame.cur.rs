extern crate glutin;
extern crate servo;

// extern crate egl;
// extern crate smallvec;

use simpleservo::MediaSessionPlaybackState;
/* use egl::egl::EGLContext;
use egl::egl::EGLDisplay;
use egl::egl::EGLSurface;
use egl::egl::MakeCurrent;
use egl::egl::SwapBuffers; */
use log::info;
use log::warn;
use servo::euclid::Scale;
use servo::keyboard_types::Key;
use servo::servo_url::ServoUrl;
use servo::webrender_api::units::{DevicePixel, DevicePoint, LayoutPixel};
use simpleservo::{self, deinit, gl_glue, MouseButton, ServoGlue, SERVO};
use simpleservo::{
    Coordinates, EventLoopWaker, HostTrait, InitOptions, PromptResult,
};
use std::cell::Cell;
use std::ffi::CStr;
use std::ffi::CString;
use std::io::Write;
use std::rc::Rc;
use std::thread;
use std::time::Duration;
use std::time::Instant;

pub struct ServoInstance {
    scroll_state: ScrollState,
    scroll_scale: Scale<f32, DevicePixel, LayoutPixel>,
    shut_down_complete: Rc<Cell<bool>>,
}

struct EventLoopWakerInstance;

impl EventLoopWaker for EventLoopWakerInstance {
    fn clone_box(&self) -> Box<dyn EventLoopWaker> {
        Box::new(EventLoopWakerInstance)
    }

    fn wake(&self) {}
}

pub fn main () {
    let gl = gl_glue::egl::init().expect("EGL initialization failure");

    let coordinates = Coordinates::new(0,0, 400, 400, 400, 400); /* Coordinates::new(
        0,
        0,
        width as i32,
        height as i32,
        width as i32,
        height as i32,
    ); */

    let mut url = "https://google.com";

    // If the URL has a space in it, then treat everything before the space as arguments
    let args = /* if let Some(i) = url.rfind(' ') {
        let (front, back) = url.split_at(i);
        url = back;
        front.split(' ').map(|s| s.to_owned()).collect()
    } else if !default_args.is_null() {
        CStr::from_ptr(default_args)
            .to_str()
            .unwrap_or("")
            .split(' ')
            .map(|s| s.to_owned())
            .collect()
    } else {
        Vec::new()
    }; */ Vec::new();

    info!("got args: {:?}", args);

    let opts = InitOptions {
        args,
        url: Some(url.to_string()),
        density: 1.0, // hidpi,
        enable_subpixel_text_antialiasing: false,
        xr_discovery: None,
        coordinates,
        gl_context_pointer: None, // Some(ctxt),
        native_display_pointer: None, // Some(disp),
        native_widget: None
    };
    let wakeup = Box::new(EventLoopWakerInstance);
    let shut_down_complete = Rc::new(Cell::new(false));
    let callbacks = Box::new(HostCallbacks {
        /* app,
        ctxt,
        surf,
        disp,
        landscape,
        shut_down_complete: shut_down_complete.clone(),
        history_update,
        url_update,
        keyboard, */
    });
    info!("Starting servo");
    simpleservo::init(opts, gl.gl_wrapper, wakeup, callbacks).expect("error initializing Servo");

    let result = ServoInstance {
        scroll_state: ScrollState::TriggerUp,
        scroll_scale: Scale::new(1.0), // SCROLL_SCALE / hidpi
        shut_down_complete,
    };

    // return result;
}

#[derive(Clone, Copy)]
enum ScrollState {
    TriggerUp,
    TriggerDown(DevicePoint),
    TriggerDragging(DevicePoint, DevicePoint),
}

struct HostCallbacks {
    /* ctxt: EGLContext,
    surf: EGLSurface,
    disp: EGLDisplay,
    landscape: bool,
    shut_down_complete: Rc<Cell<bool>>,
    history_update: MLHistoryUpdate,
    url_update: MLURLUpdate,
    app: MLApp,
    keyboard: MLKeyboard,*/
}

impl HostTrait for HostCallbacks {
    /* fn flush(&self) {
        // Immersive and landscape apps have different requirements for who calls SwapBuffers.
        if self.landscape {
            SwapBuffers(self.disp, self.surf);
        }
    }

    fn make_current(&self) {
        MakeCurrent(self.disp, self.surf, self.surf, self.ctxt);
    } */

    fn prompt_alert(&self, message: String, _trusted: bool) {
        warn!("Prompt Alert: {}", message);
    }

    fn prompt_ok_cancel(&self, message: String, _trusted: bool) -> PromptResult {
        warn!("Prompt not implemented. Cancelled. {}", message);
        PromptResult::Secondary
    }

    fn prompt_yes_no(&self, message: String, _trusted: bool) -> PromptResult {
        warn!("Prompt not implemented. Cancelled. {}", message);
        PromptResult::Secondary
    }

    fn prompt_input(&self, message: String, default: String, _trusted: bool) -> Option<String> {
        warn!("Input prompt not implemented. {}", message);
        Some(default)
    }

    fn on_load_started(&self) {}
    fn on_load_ended(&self) {}
    fn on_title_changed(&self, _title: String) {}
    fn on_allow_navigation(&self, _url: String) -> bool {
        true
    }
    fn on_url_changed(&self, url: String) {
        /* if let Ok(cstr) = CString::new(url.as_str()) {
            if let Some(url_update) = self.url_update.0 {
                url_update(self.app, cstr.as_ptr());
            }
        } */
    }

    fn on_history_changed(&self, can_go_back: bool, can_go_forward: bool) {
        /* if let Some(history_update) = self.history_update.0 {
            history_update(self.app, can_go_back, can_go_forward);
        } */
    }

    fn on_animating_changed(&self, _animating: bool) {}

    fn on_shutdown_complete(&self) {
        // self.shut_down_complete.set(true);
    }

    fn on_ime_state_changed(&self, show: bool) {
        /* if let Some(keyboard) = self.keyboard.0 {
            keyboard(self.app, show)
        } */
    }

    fn get_clipboard_contents(&self) -> Option<String> {
        None
    }

    fn set_clipboard_contents(&self, _contents: String) {}

    fn on_devtools_started(&self, port: Result<u16, ()>) {
        match port {
            Ok(p) => info!("Devtools Server running on port {}", p),
            Err(()) => error!("Error running Devtools server"),
        }
    }

    // TODO: impl

    fn show_context_menu(&self, title: Option<String>, items: Vec<String>) {

    }

    fn on_media_session_metadata(&self, title: String, artist: String, album: String) {

    }

    fn on_media_session_playback_state_change(&self, state: MediaSessionPlaybackState) {

    }

    fn on_media_session_set_position_state(&self, duration: f64, position: f64, playback_rate: f64) {

    }
}
