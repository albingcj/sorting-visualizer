use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Document, Window, HtmlElement};

pub struct SortingVisualizer;

impl SortingVisualizer {
    pub fn bubble_sort(arr: &mut Vec<i32>, visualize: impl Fn(usize, usize)) {
        let n = arr.len();
        for i in 0..n {
            for j in 0..n - i - 1 {
                if arr[j] > arr[j + 1] {
                    arr.swap(j, j + 1);
                    visualize(j, j + 1);
                }
            }
        }
    }
}

#[wasm_bindgen]
pub fn visualize_sorting_algorithm(algorithm: &str, input_array: Vec<i32>) {
    let window: Window = web_sys::window().expect("No global window exists");
    let document: Document = window.document().expect("Should have a document");
    let container: HtmlElement = document
        .get_element_by_id("visualization")
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap();

    let mut arr = input_array.clone();

    // Create visualization bars
    for &value in &arr {
        let bar = document
            .create_element("div")
            .unwrap()
            .dyn_into::<HtmlElement>()
            .unwrap();
        bar.set_class_name("bar");
        
        // Use style() instead of style
        bar.style()
            .set_property("height", &format!("{}px", value))
            .unwrap();

        container.append_child(&bar).unwrap();
    }

    // Visualization closure
    let visualize = |i: usize, j: usize| {
        // Use .children() to get the HTMLCollection
        let bars = container.children();
        let bar_i = bars.item(i as u32).unwrap().dyn_into::<HtmlElement>().unwrap();
        let bar_j = bars.item(j as u32).unwrap().dyn_into::<HtmlElement>().unwrap();

        let temp_height = bar_i.style().get_property_value("height").unwrap();
        bar_i.style()
             .set_property("height", &bar_j.style().get_property_value("height").unwrap())
             .unwrap();
        bar_j.style().set_property("height", &temp_height).unwrap();
    };

    // Execute the selected sorting algorithm
    match algorithm {
        "bubble_sort" => SortingVisualizer::bubble_sort(&mut arr, visualize),
        _ => panic!("Unsupported algorithm"),
    }
}