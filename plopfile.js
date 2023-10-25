export default function(plop) {
  plop.setGenerator('web-web-component', {
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
        pattern: /(\/\/ PLOP COMPONENT MOD)/g,
        template: 'pub mod {{snakeCase component_name}};',
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
