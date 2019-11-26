mod variable_and_mutability_example;
mod data_type_example;
mod function_example;
mod control_flow_example;
mod module_example;
mod ownership_example;
mod reference_example;
mod slice_example;
mod struct_example;

fn main() {
    // 변수와 가변성 예제
    variable_and_mutability_example::example_variable_and_mutability();

    // 데이터 타입 예제
    data_type_example::example_data_type();

    // 함수 예제
    function_example::example_function();

    // 제어문 예제
    control_flow_example::example_control_flow();

    // 소유권 예제
    ownership_example::example_ownership();

    // 참조자 예제
    reference_example::example_reference();

    // 슬라이스 예제
    slice_example::example_slices();

    // 모듈 예제
    module_example::example_module();

    // 구조체 예제
    struct_example::example_struct();
}