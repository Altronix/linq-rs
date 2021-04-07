use super::resource::*;
use crate::error::*;
use futures::channel::oneshot;
use std::collections::HashMap;
use std::sync::Arc;
use std::thread::JoinHandle;
pub use webview_official::{SizeHint, Webview};

type Bindings = HashMap<String, Box<dyn FnMut(&str, &str) + Send + 'static>>;

pub struct WindowBuilder {
    title: String,
    root: String,
    js: Vec<String>,
    bindings: Bindings,
    x: i32,
    y: i32,
}

impl Default for WindowBuilder {
    fn default() -> Self {
        WindowBuilder {
            title: "application".into(),
            root: "<html>Bad webpage</html>".into(),
            bindings: HashMap::new(),
            js: vec![],
            x: 800,
            y: 600,
        }
    }
}

impl WindowBuilder {
    pub fn new() -> Self {
        WindowBuilder::default()
    }

    pub fn title<'a>(mut self, title: &'a str) -> Self {
        self.title = title.into();
        self
    }

    pub fn size(mut self, x: i32, y: i32) -> Self {
        self.x = x;
        self.y = y;
        self
    }

    pub fn load_root<'a>(mut self, resource: Html<'a>) -> Result<Self> {
        match resource {
            Html::File(path) => self.root = Self::load_file(path)?,
            Html::Url(_) => panic!(),
        }
        Ok(self)
    }

    pub fn load_js<'a>(mut self, js: Js<'a>) -> Result<Self> {
        match js {
            Js::File(path) => self.js.push(Self::load_file(path)?),
            Js::Url(_) => panic!(),
        }
        Ok(self)
    }

    pub fn load_file<'a>(path: &'a str) -> Result<String> {
        Ok(String::from_utf8(std::fs::read(path)?)?)
    }

    pub fn bind<F: 'static>(mut self, name: &str, f: F) -> Self
    where
        F: FnMut(&str, &str) + Send,
    {
        self.bindings.insert(name.into(), Box::new(f));
        self
    }

    pub fn build(mut self) -> Result<Window> {
        let mut webview = Webview::create(true, None);
        webview.set_title(&self.title);
        webview.set_size(self.x, self.y, SizeHint::FIXED.into());
        webview.navigate(&self.root);
        for js in &self.js {
            webview.init(js);
        }
        for binding in &mut self.bindings {
            // TODO our callback is called from window thread. Use ipc
            //      to send message back to our thread.
            webview.bind(&binding.0, |seq, req| (*binding.1)(seq, req))
        }
        let th = std::thread::spawn(move || {
            webview.run();
            Ok(())
        });

        Ok(Window {
            join_handle: Some(th),
        })
    }
}

pub struct Window {
    pub join_handle: Option<JoinHandle<Result<()>>>,
}
impl Window {
    /*
    pub fn start(mut self) -> Self {
        let mut clone = Arc::clone(&self.webview);
        let th = std::thread::spawn(move || {
            let w = Arc::get_mut(&mut clone).ok_or(ViewError::Impossible(
                "dangling mutable reference to webview".to_string(),
            ))?;
            w.run();
            Ok(())
        });
        self.join_handle = Some(th);
        self
    }

    pub async fn eval<'js>(&mut self, js: &'js str) -> Result<()> {
        /*
        let js = Box::new(String::from(js));
        let (tx, rx) = oneshot::channel::<()>();
        self.webview.dispatch(move |w| {
            w.eval(&js);
            tx.send(()).expect("dangling IPC channel!");
        });
        rx.await.map_err(|_| ViewError::Ipc("Ipc cancelled".into()))
        */
        Ok(())
    }

    pub fn terminate(&mut self) {
        /*
        self.webview.terminate();
        */
    }
    */
}
