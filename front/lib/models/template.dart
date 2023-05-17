import 'package:checkit/config/enums/ETaskListStatus.dart';
import 'package:checkit/models/executable_task.dart';

class Template {
  late String name;
  late List<int> weekdays;
  late DateTime implementationDate;
  late List<ExecutableTask> listOfTasks;
  late TemplateStatus status;

  Template(
      {required this.name,
        required this.weekdays,
        required this.listOfTasks,
        required DateTime implementationDate,
        required TemplateStatus status});

  factory Template.init(String name, List<int> weekdays, List<ExecutableTask> listOfTasks){
    return Template(
      name: name,
      weekdays: weekdays,
      listOfTasks: listOfTasks,
      implementationDate: DateTime.now(),
      status: TemplateStatus.inProgress
    );
  }
}