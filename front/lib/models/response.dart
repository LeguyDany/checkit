class Response {
  late bool success;
  late dynamic data;
  late int status;

  Response({required this.success, required this.data, required this.status});

  factory Response.fromJson(Map<String, dynamic> json) {
    return Response(
        success: json['success'], data: json['data'], status: json['status']);
  }
}
