// src/main.rs - Final, Corrected Code for Window and Webview

// --- 1. Fix Imports: Use Wry's re-exports of Tao components ---
// Tao/Wry provides everything we need: EventLoop, WindowBuilder, and the Event type.
use wry::application::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    dpi::LogicalSize,
};
use wry::webview::WebViewBuilder;
use wry::webview::Url; 
use std::path::PathBuf;

// NOTE: We now change fn main to return wry::Result<()>
fn main() -> wry::Result<()> { 
    
    // 1. Setup the Tao Event Loop (Fixed by using wry::application::event_loop)
    let event_loop = EventLoop::new(); 
    
    // 2. Build the Tao Window (Fixed by using wry::application::window::WindowBuilder)
    let window = WindowBuilder::new()
        .with_title("Flowstate Productivity")
        .with_inner_size(LogicalSize::new(800, 600)) // LogicalSize is also from Tao/wry
        .build(&event_loop)
        .unwrap(); 

    // 3. Define the path to your compiled frontend assets
    let mut absolute_path = std::env::current_dir().unwrap();
    
    absolute_path.push("dist");
    absolute_path.push("index.html");

    if !absolute_path.exists() {
        eprintln!("FATAL ERROR: Frontend build not found at: {:?}", absolute_path);
        eprintln!("Please run 'npm run build' inside the 'frontend/' directory.");
        return Ok(());
    }

    // Convert the file path to a URL format
    let url = Url::from_file_path(&absolute_path).unwrap();
    
    // 4. Create the Wry Webview and attach it to the window
    // Fix 2: Now that 'window' is the correct Tao type, we can pass it here.
    let _webview = WebViewBuilder::new(window)? // Use '?' here (Fix 3)
        .with_url(&url.to_string())?
        .build()?;

    // 5. Run the Event Loop
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait; 

        match event {
            Event::WindowEvent { event, .. } => match event {
                
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                _ => (),
            },
            _ => (),
        }
    });
}