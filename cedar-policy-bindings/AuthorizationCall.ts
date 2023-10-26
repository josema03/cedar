// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { CedarValueJson } from "./CedarValueJson";
import type { EntityUidJson } from "./EntityUidJson";
import type { RecvdSlice } from "./RecvdSlice";
import type { SchemaFragment } from "./SchemaFragment";

export interface AuthorizationCall { principal: EntityUidJson | null, action: EntityUidJson, resource: EntityUidJson | null, context: Record<string, CedarValueJson>, schema: SchemaFragment | null, slice: RecvdSlice, }