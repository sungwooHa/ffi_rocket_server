#include <stdio.h>
#include <stdlib.h>
#include <thread>
#include <iostream>
#include "rust_lib.h"

using namespace std;


extern "C"
{
	enum e_rust_status {
		RUST_OK = 0,
		RUST_ERR_NULL_POINTER,
		RUST_ERR_PANICKED,
	};

	e_rust_status server_run(const char* (*get_all_data)(int), const char* (*get_data)(int, int)); 
	e_rust_status server_shutdown();
}


const char* GetAllData(int idxDataType) {
	switch (idxDataType)
	{
	case 1 :
		return "Test 1";
		break;
	case 2:
		return "Test 2";
		break;
	case 3:
		return "Test 3";
		break;
	}

	return 	R"([
{
"_KEY": 1,
"_PARENTSSTRUCTURE": 0,
"_CHILDSTRUCTURE": [1,2
]
}])";
}

const char* GetData(int idxDataType, int key) {
	switch (idxDataType)
	{
	case 1:
		return "Test 1, key 1";
		break;
	case 2:
		return "Test 2, key 2";
		break;
	case 3:
		return "Test 3, key 3";
		break;
	}


	return 	R"([
{
"_KEY": 1,
"_PARENTSSTRUCTURE": 0,
"_CHILDSTRUCTURE": [1,2
]
}])";
}


void server_start()
{
	//rocket_starter(print_func);
	switch (server_run(&GetAllData, &GetData))
	{
	case RUST_OK:
		cout << "success to run server" << endl;
		break;
	case RUST_ERR_NULL_POINTER:
		cout << "invalid callback function" << endl;
		break;
	case RUST_ERR_PANICKED:
	default:
		cout << "fail to run server" << endl;
		break;
	};
}

void server_stop()
{
	while (true);
	switch (server_shutdown())
	{
	case RUST_OK:
		cout << "success to stop server" << endl;
		break;
	case RUST_ERR_NULL_POINTER:
	case RUST_ERR_PANICKED:
	default:
		cout << "fail to stop server" << endl;
		break;
	};
}

int main() {
	
	std::thread t1(server_start);
	std::thread t2(server_stop);
	t1.join();
	t2.join();

	return 0;
}

//
//enum e_rust_status {
//	RUST_OK = 0,
//	RUST_ERR_NULL_POINTER,
//} ;
//
//typedef void(*cb_t) (void *);
//extern "C"
//{
//	e_rust_status register_cb(cb_t cb, void * arg);
//	e_rust_status call_cbs(void);
//	e_rust_status clear_cbs(void);
//}
//
//void rust_try(e_rust_status rust_status)
//{
//	switch (rust_status) {
//	case RUST_OK:
//		return;
//	case RUST_ERR_NULL_POINTER:
//		fprintf(stderr, "Rust call failed: got NULL pointer\n");
//		break;
//	default:
//		fprintf(stderr, "Rust call returned an unknown value\n");
//		break;
//	}
//	exit(EXIT_FAILURE);
//}
//
//void inc(int * counter)
//{
//	++(*counter);
//}
////
////int main(int argc, char const * const argv[])
////{
////	int counter = 0;
////	void* server_instance;
////	{
////		//rust_try(register_cb((cb_t)inc, &counter));
////		//rust_try(register_cb((cb_t)inc, &counter));
////		//rust_try(register_cb((cb_t)inc, &counter));
////		rust_try(start_server());
////		printf("%d\n", counter);
////		rust_try(call_cbs());
////		printf("%d\n", counter);
////		rust_try(clear_cbs());
////	}
////	return EXIT_SUCCESS;
////
////	//server 실행
////	//
////}
//
//
//
////extern "C" {
////	void* get_state(int(*callback)(char*));
////	int run(void* state);
////	void delete_state(void* state);
////}
////
////extern "C" int callBack(char* msg) {
////	std::cout << msg;
////	return 0;
////}
////
////int main(int argc, char** argv) {
////	auto state = get_state(&callBack);
////	if (run(state) != 0) {
////		std::cout << "Error calling callback";
////	}
////	delete_state(state);
////}