import 'package:checkit/config/globals.dart';
import 'package:checkit/controller/api/auth.dart';
import 'package:checkit/models/response.dart';
import 'package:checkit/helper/api_requests.dart';

class Auth extends AuthApi {
  Future<Response> checkIsAuth() async {
    final res = await apiCheckIsAuth();
    return res;
  }

  Future<Response> login(String username, String password) async {
    final res = await apiLogin(username, password);

    if (res.success) {
      await storage.write(key: "userToken", value: res.data);
      apiRequests.header["Authorization"] = res.data;
    }

    return res;
  }

  Future<Response> register(String username, String password) async {
    final res = await apiRegister(username, password);
    return res;
  }
}

Auth auth = Auth();