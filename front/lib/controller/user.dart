import 'package:checkit/controller/api/user.dart';
import 'package:checkit/controller/response.dart';
import '../config/globals.dart';
import '../helper/api_requests.dart';

class User extends UserApi {
  Future<Response> resetUserPassword(String newPassword, String oldPassword) async {
    final res = await apiResetUserPassword(newPassword, oldPassword);

    if (res.success) {
      await storage.write(key: "userToken", value: res.data);
      apiRequests.header["Authorization"] = res.data;
    }

    return res;
  }

  Future<Response> changeUsername(String username, String password) async {
    final res = await apiChangeUsername(username, password);

    if (res.success) {
      await storage.write(key: "userToken", value: res.data);
      apiRequests.header["Authorization"] = res.data;
    }

    return res;
  }

  Future<Response> getCurrentUser() async {
    final res = await apiGetCurrentUser();

    return res;
  }
}

User user = User();
