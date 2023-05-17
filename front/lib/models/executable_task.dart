class ExecutableTask {
  late String content;
  late bool checked;
  late DateTime dueTime;
  late DateTime creationDate;
  late String template;

  ExecutableTask(
      {required this.content,
      required this.dueTime,
      required this.template,
      required DateTime creationDate,
      required bool checked});

  factory ExecutableTask.init(
      String content, DateTime dueTime, String template) {
    return ExecutableTask(
        content: content,
        dueTime: dueTime,
        template: template,
        creationDate: DateTime.now(),
        checked: false);
  }
}
