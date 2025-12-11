fn bubble_sort(arr: &mut Vec<i32>) {
    let n = arr.len();

    for i in 0..n {
        for j in 0..n - 1 - i {
            if arr[j] > arr[j + 1] {
                let temp = arr[j];
                arr[j] = arr[j + 1];
                arr[j + 1] = temp;
                // arr.swap(j, j + 1);
            }
        }
    }
}

fn insertion_sort<T: Ord>(arr: &mut [T]) {
    let n = arr.len();
    // Начинаем со второго элемента (индекс 1), так как 0-й элемент уже "отсортирован"
    for i in 1..n {
        // Ключ для вставки
        let key = arr[i];
        // Индекс предыдущего элемента в отсортированной части
        let mut j = i - 1;

        // Передвигаем элементы arr[0..i-1], которые больше key,
        // на одну позицию вправо
        while j >= 0 && arr[j] > key {
            arr[j + 1] = arr[j];
            j -= 1;
        }
        // Вставляем key на правильное место
        arr[j + 1] = key;
    }
}
fn bubble_sort_optimized(arr: &mut Vec<i32>) {
    let n = arr.len();
    for i in 0..n {
        let mut swapped = false;

        for j in 0..n - 1 - i {
            if arr[j] > arr[j + 1] {
                let temp = arr[j];
                arr[j] = arr[j + 1];
                arr[j + 1] = temp;
                // arr.swap(j, j + 1);
                swapped = true;
            }
        }

        // если небыло ни одного свопа - то массив УЖЕ отсортирован
        if !swapped {
            break;
        }
    }
}

fn main() {
    let mut data1 = vec![5, 1, 4, 2, 8];
    bubble_sort(&mut data1);
    println!("{:?}", data1);

    let mut data2 = vec![5, 1, 4, 2, 8];
    bubble_sort_optimized(&mut data2);
    println!("{:?}", data2);
}
