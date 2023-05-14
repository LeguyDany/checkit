String getAppBarTitle(String route) {
  switch (route) {
    case '/tasks':
      return 'Tasks';
    case '/templates':
      return 'Templates';
    case '/settings':
      return 'Settings';
  }
  return 'Tasks';
}