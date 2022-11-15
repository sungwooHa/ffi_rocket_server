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

	return "..um...";
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

	return "..um...";
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