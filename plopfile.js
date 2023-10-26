export default function(plop) {
  plop.setGenerator('web-htmx-component', {
    description: 'web-htmx component',
    prompts: [
      {
        type: 'input',
        name: 'component_name',
        message: 'Component name: '
      }
    ],
    actions: [
      {
        type: 'addMany',
        destination: 'web-htmx/src/components',
        templateFiles: 'plop-templates/web-htmx-component/*.hbs',
        base: 'plop-templates/web-htmx-component',
      },
      {
        path: 'web-htmx/src/components.rs',
        pattern: /(\/\/##PLOP COMPONENT MOD##)/g,
        template: '$1\npub mod {{snakeCase component_name}};',
        type: 'modify',
      }
    ]
  });
  plop.setGenerator('web-htmx-resource', {
    description: 'web-htmx resource handlers',
    prompts: [
      {
        type: 'input',
        name: 'resource_name',
        message: 'Resource name: '
      }
    ],
    actions: [
      {
        type: 'addMany',
        destination: 'web-htmx/src/resources',
        templateFiles: 'plop-templates/web-htmx-resource/*.hbs',
        base: 'plop-templates/web-htmx-resource',
      },
      {
        path: 'web-htmx/src/resources.rs',
        pattern: /(\/\/##PLOP RESOURCE MOD HOOK##)/g,
        template: '$1\npub mod {{snakeCase resource_name}};',
        type: 'modify',
      },
      {
        path: 'web-htmx/src/lib.rs',
        pattern: /(\/\/##PLOP USE RESOURCE HOOK##)/g,
        template: '$1\nuse resources::{{snakeCase resource_name}}::{{snakeCase resource_name}}_routes;',
        type: 'modify',
      },
      {
        path: 'web-htmx/src/lib.rs',
        pattern: /(\/\/##PLOP MERGE ROUTE HOOK##)/g,
        template: '$1\n.merge({{snakeCase resource_name}}_routes(state))',
        type: 'modify',
      }
    ]
  });
  // create your generators here
  // plop.setGenerator('basics', {
  //   description: 'this is a skeleton plopfile',
  //   prompts: [], // array of inquirer prompts
  //   actions: []  // array of actions
  // });
  // controller generator
  // plop.setGenerator('controller', {
  //   description: 'application controller logic',
  //   prompts: [{
  //     type: 'input',
  //     name: 'name',
  //     message: 'controller name please'
  //   }],
  //   actions: [{
  //     type: 'add',
  //     path: 'src/{{name}}.js',
  //     templateFile: 'plop-templates/controller.hbs'
  //   }]
  // });
};
