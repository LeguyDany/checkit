import 'dart:convert';
import 'package:checkit/models/response.dart';
import 'package:checkit/helper/api_requests.dart';

abstract class AuthApi {
  Future<Response> apiCheckIsAuth() async {
    final res = await apiRequests.getRequest('/auth/check_logged_in');
    return res;
  }
  Future<Response> apiRegister(String username, String password) async {
    final Map<String, String> body = {'username': username, 'pwd': password};
    final res = await apiRequests.postRequest('/users/addUser', jsonEncode(body));

    return res;
  }

  Future<Response> apiLogin(String username, String password) async {
    final Map<String, String> body = {'username': username, 'pwd': password};
    final res = await apiRequests.postRequest('/auth/login', jsonEncode(body));

    return res;
  }
}
