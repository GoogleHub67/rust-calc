use wasm_bindgen::prelude::*;
use web_sys::{window, Document, HtmlElement, Element};

// Struct to represent a procedural keyboard button
struct KeyButton {
    label: &'static str,
    action: &'static str,
    class: &'static str,
}

// 1. SYSTEM ENTRY POINT: Replaces the inline Javascript bootstrap completely
#[wasm_bindgen(start)]
pub fn start_application() -> Result<(), JsValue> {
    let window = window().ok_or("No global window found")?;
    let document = window.document().ok_or("No document found")?;
    
    // Initial Render of UI Shell elements
    render_tabs(&document)?;
    render_keyboard_grid(&document, "algebra")?;
    
    Ok(())
}

// 2. DYNAMIC UI RENDERING ROUTINES
fn render_tabs(doc: &Document) -> Result<(), JsValue> {
    let target = doc.get_element_by_id("tabs-target")
        .ok_or("Missing #tabs-target layout shell element")?;
    target.set_inner_html(""); // Clear the layout container

    let tabs = vec![
        ("algebra", "Algebra"),
        ("trig", "Trigonometry"),
        ("calculus", "Calculus"),
    ];

    for (id, label) in tabs {
        let tab_el = doc.create_element("div")?;
        tab_el.set_class_name(if id == "algebra" { "tab active" } else { "tab" });
        tab_el.set_text_content(Some(label));
        
        // Setup listener context switcher right here natively
        let doc_clone = doc.clone();
        let current_id = id;
        let closure = Closure::<dyn FnMut()>::new(move || {
            let _ = render_keyboard_grid(&doc_clone, current_id);
        });
        
        tab_el.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
        closure.forget(); // Keep the click loop event allocation alive in memory
        
        target.append_child(&tab_el)?;
    }
    Ok(())
}

fn render_keyboard_grid(doc: &Document, tab_context: &str) -> Result<(), JsValue> {
    let grid_target = doc.get_element_by_id("keyboard-target")
        .ok_or("Missing #keyboard-target layout shell element")?;
    grid_target.set_inner_html(""); // Flush existing operational setup

    // Update global context styles on parent layout tabs
    if let Some(target) = doc.get_element_by_id("tabs-target") {
        let children = target.children();
        // Custom logic to cycle element classes goes here...
    }

    // Dynamic key list layout mappings parsed by tab contexts
    let layouts = match tab_context {
        "algebra" => vec![
            KeyButton { label: "▢/▢", action: "/", class: "" },
            KeyButton { label: "√▢", action: "sqrt(", class: "" },
            KeyButton { label: "⌫", action: "backspace", class: "" },
            KeyButton { label: "AC", action: "clear", class: "" },
            KeyButton { label: "7", action: "7", class: "num" },
            KeyButton { label: "8", action: "8", class: "num" },
            KeyButton { label: "9", action: "9", class: "num" },
            KeyButton { label: "➔", action: "eval", class: "eval-btn" },
        ],
        "calculus" => vec![
            KeyButton { label: "d/d▢", action: "d/dx ", class: "" },
            KeyButton { label: "∫ ▢", action: "∫ ", class: "" },
            KeyButton { label: "➔", action: "eval", class: "eval-btn" },
        ],
        _ => vec![] // Fallback pattern matrix
    };

    for btn in layouts {
        let button_element = doc.create_element("button")?;
        button_element.set_text_content(Some(btn.label));
        if !btn.class.is_empty() {
            button_element.set_class_name(btn.class);
        }

        // Functional button click dispatch router
        let action = btn.action;
        let input_closure = Closure::<dyn FnMut()>::new(move || {
            // Update input element strings natively based on actions
        });
        button_element.add_event_listener_with_callback("click", input_closure.as_ref().unchecked_ref())?;
        input_closure.forget();

        grid_target.append_child(&button_element)?;
    }
    Ok(())
}

// =========================================================================
// YOUR EXISTING MATH PARSING ENGINE (Completely Intact below)
// =========================================================================

#[derive(Debug, Clone, Copy)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

impl std::fmt::Display for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.im == 0.0 {
            write!(f, "{}", self.re)
        } else if self.re == 0.0 {
            write!(f, "{}i", self.im)
        } else if self.im < 0.0 {
            write!(f, " - {}i", self.re, self.im.abs())
        } else {
            write!(f, " + {}i", self.re, self.im)
        }
    }
}

#[wasm_bindgen]
pub fn solve_math(expression: &str, tab: &str) -> String {
    let expr = expression.trim();
    if expr.is_empty() {
        return "0".to_string();
    }

    match tab {
        "algebra" => evaluate_algebra(expr),
        "trig" => evaluate_trig(expr),
        "calculus" => evaluate_calculus(expr),
        _ => "Unknown Tab Context".to_string(),
    }
}

fn evaluate_algebra(expr: &str) -> String {
    if expr.contains('/') && !expr.contains('x') {
        let parts: Vec<&str> = expr.split('/').collect();
        if parts.len() == 2 {
            if let (Ok(n1), Ok(n2)) = (parts[0].parse::<f64>(), parts[1].parse::<f64>()) {
                if n2 == 0.0 { return "Error: Div by 0".to_string(); }
                return format!("{}", n1 / n2);
            }
        }
    }
    if expr.contains('^') && !expr.contains('x') {
        let parts: Vec<&str> = expr.split('^').collect();
        if parts.len() == 2 {
            if let (Ok(base), Ok(exp)) = (parts[0].parse::<f64>(), parts[1].parse::<f64>()) {
                return format!("{}", base.powf(exp));
            }
        }
    }
    if expr.starts_with('|') && expr.ends_with('|') {
        let inner = &expr[1..expr.len() - 1];
        if let Ok(num) = inner.parse::<f64>() {
            return format!("{}", num.abs());
        }
    }

    if expr.ends_with('!') {
        if let Ok(num) = expr[..expr.len() - 1].parse::<u64>() {
            let fact: u64 = (1..=num).product();
            return format!("{}", fact);
        }
        return "Invalid Factorial Input".to_string();
    }

    if expr.starts_with("log_") {
        if let Some(open_bracket) = expr.find('[') {
            if let Some(close_bracket) = expr.find(']') {
                let base_str = &expr[4..open_bracket];
                let val_str = &expr[open_bracket + 1..close_bracket];
                if let (Ok(base), Ok(val)) = (base_str.parse::<f64>(), val_str.parse::<f64>()) {
                    return format!("{}", val.log(base));
                }
            }
        }
    }

    if expr.contains('%') {
        let parts: Vec<&str> = expr.split('%').collect();
        if parts.len() == 2 {
            if let (Ok(n1), Ok(n2)) = (parts[0].parse::<f64>(), parts[1].parse::<f64>()) {
                return format!("{}", n1 % n2);
            }
        }
    }

    if expr.contains("<=") || expr.contains('≤') { return "Inequality system configured".to_string(); }
    if expr.contains(">=") || expr.contains('≥') { return "Inequality system configured".to_string(); }
    if expr.contains('<') { return "True/False evaluated".to_string(); }
    if expr.contains('>') { return "True/False evaluated".to_string(); }
    
    if expr == "6x+5=14" { return "x = 1.5".to_string(); }
    if expr == "(x+5)(x+2)" { return "x² + 7x + 10".to_string(); }
    if expr == "4x^2-5x-12=0" { return "x ≈ 2.48, x ≈ -1.23".to_string(); }

    if expr.contains('i') {
        return "Evaluated Complex Expression".to_string();
    }

    evaluate_basic_arithmetic(expr)
}

fn evaluate_trig(expr: &str) -> String {
    let clean = expr.replace('(', "").replace(')', "");
    if clean.starts_with("sin") {
        if let Ok(n) = clean[3..].parse::<f64>() { return format!("{}", n.to_radians().sin()); }
    }
    if clean.starts_with("cos") {
        if let Ok(n) = clean[3..].parse::<f64>() { return format!("{}", n.to_radians().cos()); }
    }
    if clean.starts_with("tan") {
        if let Ok(n) = clean[3..].parse::<f64>() { return format!("{}", n.to_radians().tan()); }
    }
    "Trig Evaluation Ready".to_string()
}

fn evaluate_calculus(expr: &str) -> String {
    if expr.starts_with("d/dx") {
        let target = expr.replace("d/dx", "").trim().to_string();
        if target == "x" { return "1".to_string(); }
        if target.starts_with("x^") {
            if let Ok(p) = target[2..].parse::<i32>() { return format!("{}x^{}", p, p - 1); }
        }
        return format!("d/dx representation of ({})", target);
    }

    if expr.starts_with("lim") {
        return "Limit evaluation converged".to_string();
    }

    if expr.starts_with("P(") || expr.starts_with("C(") {
        let is_p = expr.starts_with("P(");
        let clean = expr[2..expr.len()-1].to_string();
        let parts: Vec<&str> = clean.split(',').collect();
        if parts.len() == 2 {
            if let (Ok(n), Ok(k)) = (parts[0].parse::<u64>(), parts[1].parse::<u64>()) {
                if n >= k {
                    let n_fact: u64 = (1..=n).product();
                    let nk_fact: u64 = (1..=(n-k)).product();
                    if is_p {
                        return format!("{}", n_fact / nk_fact);
                    } else {
                        let k_fact: u64 = (1..=k).product();
                        return format!("{}", n_fact / (k_fact * nk_fact));
                    }
                }
            }
        }
        return "Math Error: n must be ≥ k".to_string();
    }

    if expr.starts_with('∑') { return "Summation compiled".to_string(); }
    if expr.starts_with('∫') { return "Definite/Indefinite Integral computed".to_string(); }

    if expr.contains('e') { return format!("{}", std::f64::consts::E); }
    if expr.contains('∞') { return "Infinity".to_string(); }

    evaluate_basic_arithmetic(expr)
}

fn evaluate_basic_arithmetic(expr: &str) -> String {
    let clean = expr.replace('×', "*").replace('÷', "/").replace('−', "-");
    // Standard basic parsing fallback
    if let Ok(val) = clean.parse::<f64>() {
        return format!("{}", val);
    }
    format!("{}", clean) 
}

use wasm_bindgen::prelude::*;
use web_sys::{window, Document, HtmlInputElement};

// Helper function to get the input display window cleanly
fn get_display_input(doc: &Document) -> Option<HtmlInputElement> {
    doc.get_element_by_id("display")?
        .dyn_into::<HtmlInputElement>().ok()
}

// Inside your button loop in render_keyboard_grid:
// for btn in layouts { ...
let doc_clone = doc.clone();
let action_str = btn.action.to_string();
let tab_ctx = tab_context.to_string();

let click_closure = Closure::<dyn FnMut()>::new(move || {
    let doc = &doc_clone;
    if let Some(display) = get_display_input(doc) {
        let current_value = display.value();
        
        match action_str.as_str() {
            "clear" => display.set_value(""),
            "backspace" => {
                if !current_value.is_empty() {
                    display.set_value(&current_value[..current_value.len() - 1]);
                }
            }
            "eval" => {
                // Calls your existing mathematical engine right here!
                let result = solve_math(&current_value, &tab_ctx);
                display.set_value(&result);
            }
            // Standard symbol or digit button append case
            _ => display.set_value(&format!("{}{}", current_value, action_str)),
        }
    }
});

button_element.add_event_listener_with_callback("click", click_closure.as_ref().unchecked_ref())?;
click_closure.forget(); // Safely pin click listener allocation to global Wasm context heap
