# 2. everything is a resource

Date: 2023-10-19

## Status

Accepted

## Context

When going over the structure of web-htmx, I initially split the concepts into

- Components
- Pages
- Resources

With `Resources` attempting to specifically using the language of REST (REsource State Transfer) and the idea being all the HTTP methods/routes for a given resource (a worksite, a worker, etc) would live here.

`Components` would be all the reusable html bits, and `Pages` would be the "frontend" concept of a "Page"

I was talking with Paul about whether "Wallchart" made sense as a frontend only concept, so there is a wallchart page, but the resource used to hydrate it is the "Worksite." The whole app is a wallchart, so it makes sense for it to exist as a core concept in our code, but what we have learned from the POC is that the Wallchart is kind of a projection of a Worksite.

So do we have a Wallchart page and Worksite resources? 

## Decision

We decided that the way REST wants us to do this, is to make "everything" a resource (if a browser cares about it). So we can *make* a wallchart resource, and GET `/wallchart/:id` would have it's own http handlers, we just might choose to implement them with our `WorksiteService`. We will likely also have a Worksite resource, since a GET to `/worksite/:id` likely has different meaning to the user than the Wallchart!

This decision feels like it better aligns with RESTful concepts, better aligns with the user's mental model, improves our use of language, and slightly simplifies the arch (by elminating the concept of Pages and further constraining the definition of "frontend specific concepts").

## Consequences

To reiterate above:

This decision feels like it better aligns with RESTful concepts, better aligns with the user's mental model, improves our use of language, and slightly simplifies the arch (by elminating the concept of Pages and further constraining the definition of "frontend specific concepts").

Risks:

We are still early in figuring this out, so as we iterate, we will develop a better understanding of this arch and these concepts and that may either validate this decision or cause us to pivot again.
