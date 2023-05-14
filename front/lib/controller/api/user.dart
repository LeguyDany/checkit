import 'dart:convert';

import 'package:checkit/helper/api_requests.dart';

import '../response.dart';

abstract class UserApi {
  Future<Response> apiGetCurrentUser(String username) async {
    final res = await apiRequests.getRequest('users/getAllUsers');
    return res;
  }
}
