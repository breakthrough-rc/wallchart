# 4. CSV uploads

Date: 2023-11-22

## Status

Proprosed

## Context

Users will likely be onboarding not from scratch, but from an existing set up. And that setup is probably google sheets.

To get up and running quickly, we would like to be able to get some kind of upload of an existing worksite from users to get them going quickly.

In the POC we had both a worksite wizard and a CSV upload, though both were a little bit lacking in polish but they seemed to do the job.

## Decision

Planning on starting off with another CSV upload, but this time with a better UX to help reduce errors/friction in onboarding.

In particular, I am looking into a few different tools/resources:

### CSV parsing

- https://github.com/BurntSushi/rust-csv
- I thought there were more options but looks like its just this

### File uploads

- https://docs.rs/axum/latest/axum/extract/struct.Multipart.html

## Consequences

I don't think there are many consqeuences here other than just it might be hard to make this "really good." CSV processing and uploads are just painful in general.

Also users will need to massage their data into the shape we want for upload -- so the real advtange of this is avoiding using the GUI to manually add everything one at a time.
