use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Document, Window, HtmlElement};

pub struct SortingVisualizer;

impl SortingVisualizer {
    ///////////////////////////////////////////////////////////////////////////
    // Bubble Sort
    ///////////////////////////////////////////////////////////////////////////
    pub fn bubble_sort(arr: &mut Vec<i32>, mut visualize: impl FnMut(usize, usize)) {
        let n = arr.len();
        for _ in 0..n {
            for j in 0..n - 1 {
                if arr[j] > arr[j + 1] {
                    arr.swap(j, j + 1);
                    visualize(j, j + 1);
                }
            }
        }
    }

    ///////////////////////////////////////////////////////////////////////////
    // Insertion Sort (using neighbor swaps)
    ///////////////////////////////////////////////////////////////////////////
    pub fn insertion_sort_swaps(arr: &mut Vec<i32>, mut visualize: impl FnMut(usize, usize)) {
        let n = arr.len();
        for i in 1..n {
            let mut j = i;
            while j > 0 && arr[j - 1] > arr[j] {
                arr.swap(j - 1, j);
                visualize(j - 1, j);
                j -= 1;
            }
        }
    }

    ///////////////////////////////////////////////////////////////////////////
    // Selection Sort
    ///////////////////////////////////////////////////////////////////////////
    pub fn selection_sort(arr: &mut Vec<i32>, mut visualize: impl FnMut(usize, usize)) {
        let n = arr.len();
        for i in 0..n {
            let mut min_index = i;
            for j in (i + 1)..n {
                if arr[j] < arr[min_index] {
                    min_index = j;
                }
            }
            if min_index != i {
                arr.swap(i, min_index);
                visualize(i, min_index);
            }
        }
    }

    ///////////////////////////////////////////////////////////////////////////
    // Merge Sort
    ///////////////////////////////////////////////////////////////////////////
    pub fn merge_sort(arr: &mut Vec<i32>, mut visualize: impl FnMut(usize, usize)) {
        fn merge_sort_recursive(
            arr: &mut [i32],
            left: usize,
            right: usize,
            visualize: &mut impl FnMut(usize, usize),
        ) {
            if left >= right {
                return;
            }
            let mid = (left + right) / 2;
            merge_sort_recursive(arr, left, mid, visualize);
            merge_sort_recursive(arr, mid + 1, right, visualize);
            merge(arr, left, mid, right, visualize);
        }

        fn merge(
            arr: &mut [i32],
            left: usize,
            mid: usize,
            right: usize,
            _visualize: &mut impl FnMut(usize, usize),
        ) {
            // If you'd like to visualize merges, you could call your closure here
            // with the indices being merged. But for now, we won't call it to avoid
            // out-of-bounds on partial merges. 
            let mut temp = Vec::new();
            let mut i = left;
            let mut j = mid + 1;

            while i <= mid && j <= right {
                if arr[i] <= arr[j] {
                    temp.push(arr[i]);
                    i += 1;
                } else {
                    temp.push(arr[j]);
                    j += 1;
                }
            }
            while i <= mid {
                temp.push(arr[i]);
                i += 1;
            }
            while j <= right {
                temp.push(arr[j]);
                j += 1;
            }

            for (k, &val) in temp.iter().enumerate() {
                arr[left + k] = val;
                // Example (commented out):
                // _visualize(left + k, left + k);
            }
        }

        let n = arr.len();
        if n > 1 {
            merge_sort_recursive(arr, 0, n - 1, &mut visualize);
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

    // Make a mutable copy of the input array
    let mut arr = input_array.clone();

    // Instead of calling container.children() each time, store the bars in a Vec
    let mut bars = Vec::new();
    for &value in &arr {
        let bar = document
            .create_element("div")
            .unwrap()
            .dyn_into::<HtmlElement>()
            .unwrap();
        bar.set_class_name("bar");
        bar.style()
            .set_property("height", &format!("{}px", value))
            .unwrap();
        container.append_child(&bar).unwrap();
        bars.push(bar);
    }

    // Visualization closure using our bars Vec
    let mut visualize = |i: usize, j: usize| {
        // Defensive check (avoid out-of-bounds panic)
        if i < bars.len() && j < bars.len() {
            let bar_i = &bars[i];
            let bar_j = &bars[j];
            let temp_height = bar_i.style().get_property_value("height").unwrap();
            bar_i
                .style()
                .set_property("height", &bar_j.style().get_property_value("height").unwrap())
                .unwrap();
            bar_j
                .style()
                .set_property("height", &temp_height)
                .unwrap();
        }
    };

    // Execute the selected sorting algorithm
    match algorithm {
        "bubble_sort" => SortingVisualizer::bubble_sort(&mut arr, &mut visualize),
        "insertion_sort" => SortingVisualizer::insertion_sort_swaps(&mut arr, &mut visualize),
        "selection_sort" => SortingVisualizer::selection_sort(&mut arr, &mut visualize),
        "merge_sort" => SortingVisualizer::merge_sort(&mut arr, &mut visualize),
        _ => panic!("Unsupported algorithm"),
    }
}