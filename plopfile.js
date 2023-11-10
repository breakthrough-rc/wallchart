export default function(plop) {
  plop.setGenerator("web-htmx-component", {
    description: "web-htmx component",
    prompts: [
      {
        type: "input",
        name: "component_name",
        message: "Component name: ",
      },
    ],
    actions: [
      {
        type: "addMany",
        destination: "web-htmx/src/components",
        templateFiles: "plop-templates/web-htmx-component/*.hbs",
        base: "plop-templates/web-htmx-component",
      },
      {
        path: "web-htmx/src/components.rs",
        pattern: /(\/\/##PLOP COMPONENT MOD##)/g,
        template: "$1\npub mod {{snakeCase component_name}};",
        type: "modify",
      },
    ],
  });
  plop.setGenerator("web-htmx-resource", {
    description: "web-htmx resource handlers",
    prompts: [
      {
        type: "input",
        name: "resource_name",
        message: "Resource name (plural): ",
      },
    ],
    actions: [
      {
        type: "addMany",
        destination: "web-htmx/src/resources",
        templateFiles: "plop-templates/web-htmx-resource/*.hbs",
        base: "plop-templates/web-htmx-resource",
      },
      {
        path: "web-htmx/src/resources.rs",
        pattern: /(\/\/##PLOP RESOURCE MOD HOOK##)/g,
        template: "$1\npub mod {{snakeCase resource_name}};",
        type: "modify",
      },
      {
        path: "web-htmx/src/lib.rs",
        pattern: /(\/\/##PLOP USE RESOURCE HOOK##)/g,
        template:
          "$1\nuse resources::{{snakeCase resource_name}}::{{snakeCase resource_name}}_routes;",
        type: "modify",
      },
      {
        path: "web-htmx/src/lib.rs",
        pattern: /(\/\/##PLOP MERGE ROUTE HOOK##)/g,
        template: "$1\n.merge({{snakeCase resource_name}}_routes(state))",
        type: "modify",
      },
    ],
  });
  plop.setGenerator("service-command", {
    description: "Create a command for a service",
    prompts: [
      {
        type: "input",
        name: "service_name",
        message: "Service name: ",
      },
      {
        type: "input",
        name: "command_name",
        message: "Command (usecase) name: ",
      },
    ],
    actions: [
      {
        type: "addMany",
        destination:
          "{{kabobCase service_name}}/{{kabobCase service_name}}-service/src",
        templateFiles: "plop-templates/service-command/*.hbs",
        base: "plop-templates/service-command",
      },
      {
        path: "{{kabobCase service_name}}/{{kabobCase service_name}}-service/src/lib.rs",
        pattern: /(\/\/##PLOP INSERT MOD HOOK##)/g,
        template: "$1\npub mod {{snakeCase command_name}};",
        type: "modify",
      },
      {
        path: "{{kabobCase service_name}}/{{kabobCase service_name}}-service/src/service.rs",
        pattern: /(\/\/##PLOP INSERT COMMAND IMPORTS HOOK##)/g,
        template: `$1
    {{snakeCase command_name}}::{
      {{pascalCase command_name}}, {{pascalCase command_name}}Input, {{pascalCase command_name}}Output, 
    },`,
        type: "modify",
      },
      {
        path: "{{kabobCase service_name}}/{{kabobCase service_name}}-service/src/service.rs",
        pattern: /(\/\/##PLOP INSERT COMMAND HOOK##)/g,
        template: `$1
    pub {{snakeCase command_name}}: {{pascalCase command_name}},`,
        type: "modify",
      },
      {
        path: "{{kabobCase service_name}}/{{kabobCase service_name}}-service/src/service.rs",
        pattern: /(\/\/##PLOP INSERT COMMAND INSTANTIATION HOOK##)/g,
        template: `$1
            {{snakeCase command_name}}: {{pascalCase command_name}} {
              // Add any dependencies for the command here. They should be passed into this function and supplied by main.rs.
            },`,
        type: "modify",
      },
      {
        path: "{{kabobCase service_name}}/{{kabobCase service_name}}-service/src/service.rs",
        pattern: /(\/\/##PLOP INSERT DELEGATE HOOK##)/g,
        template: `$1
    pub async fn {{snakeCase command_name}}(
        &self,
        input: {{pascalCase command_name}}Input,
    ) -> {{pascalCase command_name}}Output {
        self.{{snakeCase command_name}}.{{snakeCase command_name}}(input).await
    }
`,
        type: "modify",
      },
      // {
      //   path: 'web-htmx/src/lib.rs',
      //   pattern: /(\/\/##PLOP USE RESOURCE HOOK##)/g,
      //   template: '$1\nuse resources::{{snakeCase resource_name}}::{{snakeCase resource_name}}_routes;',
      //   type: 'modify',
      // },
      // {
      //   path: 'web-htmx/src/lib.rs',
      //   pattern: /(\/\/##PLOP MERGE ROUTE HOOK##)/g,
      //   template: '$1\n.merge({{snakeCase resource_name}}_routes(state))',
      //   type: 'modify',
      // }
    ],
  });
  plop.setGenerator("service-skeleton", {
    description: "Create the skeleton of a new service",
    prompts: [
      {
        type: "input",
        name: "service_name",
        message: "Service name: ",
      },
    ],
    actions: [
      {
        type: "addMany",
        destination: "",
        templateFiles: "plop-templates/service-skeleton/*.hbs",
        base: "plop-templates/service-skeleton",
      },
    ],
  });
}
