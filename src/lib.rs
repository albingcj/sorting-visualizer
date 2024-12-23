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

    pub fn merge_sort(arr: &mut Vec<i32>, mut visualize: impl FnMut(usize, usize)) {
        fn merge_sort_recursive(
            arr: &mut [i32],
            offset: usize,
            visualize: &mut impl FnMut(usize, usize),
        ) {
            let n = arr.len();
            if n <= 1 {
                return;
            }
            let mid = n / 2;
            
            merge_sort_recursive(&mut arr[..mid], offset, visualize);
            merge_sort_recursive(&mut arr[mid..], offset + mid, visualize);
            merge(arr, offset, mid, visualize);
        }
    
        fn merge(
            arr: &mut [i32],
            offset: usize,
            mid: usize,
            visualize: &mut impl FnMut(usize, usize),
        ) {
            let left_part = arr[..mid].to_vec();
            let right_part = arr[mid..].to_vec();
    
            let (mut i, mut j, mut k) = (0, 0, 0);
            
            while i < left_part.len() && j < right_part.len() {
                if left_part[i] <= right_part[j] {
                    arr[k] = left_part[i];
                    // Swap the bar from "left side index" to the merged position k
                    // 'offset + i' is where the left element originally came from
                    visualize(offset + i, offset + k);
                    i += 1;
                } else {
                    arr[k] = right_part[j];
                    // Swap the bar from "right side index" (offset+mid+j) to position k
                    visualize(offset + mid + j, offset + k);
                    j += 1;
                }
                k += 1;
            }
    
            // Copy any remaining elements of left_part
            while i < left_part.len() {
                arr[k] = left_part[i];
                visualize(offset + i, offset + k);
                i += 1;
                k += 1;
            }
    
            // Copy any remaining elements of right_part
            while j < right_part.len() {
                arr[k] = right_part[j];
                visualize(offset + mid + j, offset + k);
                j += 1;
                k += 1;
            }
        }
    
        merge_sort_recursive(arr, 0, &mut visualize);
    }

    ///////////////////////////////////////////////////////////////////////////
    // Quick Sort (added)
    ///////////////////////////////////////////////////////////////////////////
    pub fn quick_sort(arr: &mut Vec<i32>, mut visualize: impl FnMut(usize, usize)) {
        fn quick_sort_recursive(
            arr: &mut [i32],
            offset: usize,
            visualize: &mut impl FnMut(usize, usize),
        ) {
            let len = arr.len();
            if len <= 1 {
                return;
            }

            // Partition the array
            let pivot_index = partition(arr, offset, visualize);
            // Recursively sort elements before pivot
            let (left, right) = arr.split_at_mut(pivot_index);
            quick_sort_recursive(left, offset, visualize);
            // Recursively sort elements after pivot
            if right.len() > 1 {
                quick_sort_recursive(&mut right[1..], offset + pivot_index + 1, visualize);
            }
        }

        fn partition(
            arr: &mut [i32],
            offset: usize,
            visualize: &mut impl FnMut(usize, usize),
        ) -> usize {
            let pivot_value = arr[arr.len() - 1];
            let mut i = 0; 
            for j in 0..arr.len() - 1 {
                if arr[j] < pivot_value {
                    arr.swap(i, j);
                    visualize(offset + i, offset + j);
                    i += 1;
                }
            }
            arr.swap(i, arr.len() - 1);
            visualize(offset + i, offset + arr.len() - 1);
            i
        }

        quick_sort_recursive(arr, 0, &mut visualize);
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
        "quick_sort" => SortingVisualizer::quick_sort(&mut arr, &mut visualize),
        _ => panic!("Unsupported algorithm"),
    }
}
