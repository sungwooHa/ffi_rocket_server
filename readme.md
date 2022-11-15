# rust server for C++

Handling rocket instances in C++

- rocket API
- ffi

## crate
- tokio
- rocket
- reqwest


# ffi
- check example file([example/ffi_cpp_main.cpp](/example/ffi_cpp_main.cpp))
- 사용할 떄, RAII를 이용해서 소멸자에서 shutdown을 호출하도록 만들면, instance 관리가 쉬울 거승로 예상한다.


# contetns
## error handling
`catch_unwind`를 통한 error handling.

`e_rust_status` 반환값을 통해서 상태를 확인할 수 있다.

## 특징
- rocket의 instance는 관리를 제공하지 않기 때문에 instance를 C++로 넘겨 줄 수 없다.
- 내부에서 rocket shutdown은 제공하지 않기 때문에, local로 reqwest를 보내는 방식으로 shutdown 을 호출한다.
- 

