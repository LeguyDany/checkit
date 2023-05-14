import 'dart:convert';
import 'package:checkit/helper/api_requests.dart';
import '../response.dart';

abstract class UserApi {
  Future<Response> apiResetUserPassword(String newPassword, String oldPassword) async {
    final Map<String, dynamic> body = {'updated_value': newPassword, 'pwd': oldPassword, 'is_username': false};
    final res = await apiRequests.patchRequest('users/update_user', jsonEncode(body));
    return res;
  }

  Future<Response> apiChangeUsername(String newUsername, String password) async {
    final Map<String, dynamic> body = {'updated_value': newUsername, 'pwd': password, 'is_username': true};
    final res = await apiRequests.patchRequest('users/update_user', jsonEncode(body));
    return res;
  }
  Future<Response> apiGetCurrentUser() async {
    final res = await apiRequests.getRequest('users/getCurrentUser');
    return res;
  }
}
