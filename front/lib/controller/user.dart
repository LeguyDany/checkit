import 'dart:convert';
import 'package:checkit/controller/api/user.dart';
import 'package:checkit/controller/response.dart';

class User extends UserApi {
  Future<Map<String, dynamic>> getCurrentUser(String username) async {
    final res = await apiGetCurrentUser(username);
    final filtered = res.data
        .where((i) => i["username"] == username).toList()[0];

    return filtered;
  }
}

User user = User();
