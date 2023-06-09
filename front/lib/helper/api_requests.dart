import 'dart:convert';
import 'package:checkit/models/response.dart';
import 'package:http/http.dart' as http;
import 'package:flutter_dotenv/flutter_dotenv.dart';

class ApiRequests {
  final String apiAddress = dotenv.env['API_ADDRESS']!;
  final String apiPort = dotenv.env['API_PORT']!;
  late Map<String, dynamic> header;

  Future<Response> patchRequest(String route, dynamic body) async {
    var url = Uri.http('$apiAddress:$apiPort', route);
    var response = await http.patch(url, body: body, headers: <String, String>{
      'Authorization': 'Bearer ${header["Authorization"]}',
      'Content-Type': 'application/json; charset=UTF-8',
    });
    final Map<String, dynamic> itemJson = jsonDecode(response.body);
    final castedResponse = Response.fromJson(itemJson);

    return castedResponse;
  }

  Future<Response> postRequest(String route, dynamic body) async {
    var url = Uri.http('$apiAddress:$apiPort', route);
    var response = await http.post(url, body: body, headers: <String, String>{
      'Content-Type': 'application/json; charset=UTF-8',
    });
    final Map<String, dynamic> itemJson = jsonDecode(response.body);
    final castedResponse = Response.fromJson(itemJson);

    return castedResponse;
  }

  Future<Response> getRequest(String route) async {
    var url = Uri.http('$apiAddress:$apiPort', route);
    var response = await http.get(url,
        headers: {'Authorization': 'Bearer ${header["Authorization"]}'});
    final Map<String, dynamic> itemJson = jsonDecode(response.body);
    final castedResponse = Response.fromJson(itemJson);

    return castedResponse;
  }
}

ApiRequests apiRequests = ApiRequests();
