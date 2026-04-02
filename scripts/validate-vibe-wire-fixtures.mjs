#!/usr/bin/env node

import { mkdtemp, mkdir, readFile, rm, writeFile } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import path from 'node:path';
import process from 'node:process';
import { fileURLToPath, pathToFileURL } from 'node:url';

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(scriptDir, '..');
const fixtureDir = path.join(repoRoot, 'crates', 'vibe-wire', 'fixtures');
const happyWireDir = '/root/happy/packages/happy-wire/src';
const happyAppMessageMetaPath = '/root/happy/packages/happy-app/sources/sync/typesMessageMeta.ts';

const bridgeSources = {
  messageMeta: path.join(happyWireDir, 'messageMeta.ts'),
  legacyProtocol: path.join(happyWireDir, 'legacyProtocol.ts'),
  sessionProtocol: path.join(happyWireDir, 'sessionProtocol.ts'),
  messages: path.join(happyWireDir, 'messages.ts'),
  voice: path.join(happyWireDir, 'voice.ts'),
  appMessageMeta: happyAppMessageMetaPath,
};

const fixtureFiles = {
  messageMeta: 'message-meta.json',
  legacyMessages: 'legacy-messages.json',
  sessionEnvelopes: 'session-envelopes.json',
  messageContent: 'message-content.json',
  updateContainers: 'update-containers.json',
  voiceResponses: 'voice-responses.json',
  invalidSessionEnvelopes: 'session-invalid-envelopes.json',
};

async function main() {
  const bridgeDir = await mkdtemp(path.join(tmpdir(), 'vibe-wire-happy-bridge-'));

  try {
    await buildBridgeModules(bridgeDir);
    const modules = await loadBridgeModules(bridgeDir);
    await validatePublishedFixtures(modules);
    validateOptionalNullRejections(modules);

    console.log('Validated vibe-wire fixtures against Happy schemas');
  } finally {
    await rm(bridgeDir, { recursive: true, force: true });
  }
}

async function buildBridgeModules(bridgeDir) {
  await mkdir(bridgeDir, { recursive: true });
  const packageImports = {
    zod: import.meta.resolve('zod'),
    cuid2: import.meta.resolve('@paralleldrive/cuid2'),
  };

  for (const [moduleName, sourcePath] of Object.entries(bridgeSources)) {
    const source = await readFile(sourcePath, 'utf8');
    const transformed = transformTypeScriptModule(source, packageImports);
    await writeFile(path.join(bridgeDir, `${moduleName}.mjs`), transformed);
  }
}

async function loadBridgeModules(bridgeDir) {
  const modules = {};

  for (const moduleName of Object.keys(bridgeSources)) {
    const moduleUrl = pathToFileURL(path.join(bridgeDir, `${moduleName}.mjs`)).href;
    modules[moduleName] = await import(moduleUrl);
  }

  return modules;
}

function transformTypeScriptModule(source, packageImports) {
  return source
    .replace(/from 'zod'/g, `from '${packageImports.zod}'`)
    .replace(/from '@paralleldrive\/cuid2'/g, `from '${packageImports.cuid2}'`)
    .replace(/from '(\.\/[^']+)'/g, (_, relativePath) => `from '${relativePath}.mjs'`)
    .replace(/,\s*type\s+[A-Za-z0-9_]+/g, '')
    .replace(/type\s+([A-Za-z0-9_]+)\s*,/g, '$1,')
    .replace(/export type CreateEnvelopeOptions = \{[\s\S]*?\n\};\n\n/, '')
    .replace(/^export type .*;\n/gm, '')
    .replace(
      'export function createEnvelope(role: SessionRole, ev: SessionEvent, opts: CreateEnvelopeOptions = {}): SessionEnvelope {',
      'export function createEnvelope(role, ev, opts = {}) {',
    );
}

async function validatePublishedFixtures(modules) {
  const messageMetaFixtures = await readFixtureFile(fixtureFiles.messageMeta);
  const legacyMessageFixtures = await readFixtureFile(fixtureFiles.legacyMessages);
  const sessionEnvelopeFixtures = await readFixtureFile(fixtureFiles.sessionEnvelopes);
  const messageContentFixtures = await readFixtureFile(fixtureFiles.messageContent);
  const updateContainerFixtures = await readFixtureFile(fixtureFiles.updateContainers);
  const voiceResponseFixtures = await readFixtureFile(fixtureFiles.voiceResponses);
  const invalidSessionEnvelopeFixtures = await readFixtureFile(
    fixtureFiles.invalidSessionEnvelopes,
  );

  validateFixtures(
    messageMetaFixtures,
    modules.appMessageMeta.MessageMetaSchema,
    'happy-app MessageMetaSchema',
  );
  validateFixtures(
    messageMetaFixtures.filter(({ name }) => name !== 'message-meta-custom-permission-mode'),
    modules.messageMeta.MessageMetaSchema,
    'happy-wire MessageMetaSchema',
  );

  const customPermissionFixture = messageMetaFixtures.find(
    ({ name }) => name === 'message-meta-custom-permission-mode',
  );
  assert(customPermissionFixture, 'missing message-meta-custom-permission-mode fixture');
  expectReject(
    modules.messageMeta.MessageMetaSchema,
    customPermissionFixture.value,
    'happy-wire MessageMetaSchema custom permissionMode fixture',
  );

  validateFixtures(
    legacyMessageFixtures,
    modules.legacyProtocol.LegacyMessageContentSchema,
    'happy-wire LegacyMessageContentSchema',
  );
  validateFixtures(
    sessionEnvelopeFixtures,
    modules.sessionProtocol.sessionEnvelopeSchema,
    'happy-wire sessionEnvelopeSchema',
  );
  validateFixtures(
    messageContentFixtures,
    modules.messages.MessageContentSchema,
    'happy-wire MessageContentSchema',
  );
  validateFixtures(
    updateContainerFixtures,
    modules.messages.CoreUpdateContainerSchema,
    'happy-wire CoreUpdateContainerSchema',
  );
  validateFixtures(
    voiceResponseFixtures,
    modules.voice.VoiceTokenResponseSchema,
    'happy-wire VoiceTokenResponseSchema',
  );

  for (const fixture of invalidSessionEnvelopeFixtures) {
    expectReject(
      modules.sessionProtocol.sessionEnvelopeSchema,
      fixture.value,
      `happy-wire sessionEnvelopeSchema invalid fixture ${fixture.name}`,
    );
  }
}

function validateOptionalNullRejections(modules) {
  expectReject(
    modules.appMessageMeta.MessageMetaSchema,
    { sentFrom: null },
    'happy-app MessageMetaSchema sentFrom=null',
  );
  expectReject(
    modules.appMessageMeta.MessageMetaSchema,
    { permissionMode: null },
    'happy-app MessageMetaSchema permissionMode=null',
  );
  expectReject(
    modules.legacyProtocol.UserMessageSchema,
    {
      role: 'user',
      content: {
        type: 'text',
        text: 'hello',
      },
      localKey: null,
    },
    'happy-wire UserMessageSchema localKey=null',
  );
  expectReject(
    modules.legacyProtocol.AgentMessageSchema,
    {
      role: 'agent',
      content: {
        type: 'output',
      },
      meta: null,
    },
    'happy-wire AgentMessageSchema meta=null',
  );
  expectReject(
    modules.sessionProtocol.sessionEventSchema,
    {
      t: 'text',
      text: 'hello',
      thinking: null,
    },
    'happy-wire sessionEventSchema text.thinking=null',
  );
  expectReject(
    modules.sessionProtocol.sessionEventSchema,
    {
      t: 'file',
      ref: 'upload-1',
      name: 'report.txt',
      size: 1,
      mimeType: null,
    },
    'happy-wire sessionEventSchema file.mimeType=null',
  );
  expectReject(
    modules.sessionProtocol.sessionEventSchema,
    {
      t: 'start',
      title: null,
    },
    'happy-wire sessionEventSchema start.title=null',
  );
  expectReject(
    modules.sessionProtocol.sessionEnvelopeSchema,
    {
      id: 'msg-1',
      time: 1,
      role: 'agent',
      turn: null,
      ev: {
        t: 'text',
        text: 'hello',
      },
    },
    'happy-wire sessionEnvelopeSchema turn=null',
  );
  expectReject(
    modules.sessionProtocol.sessionEnvelopeSchema,
    {
      id: 'msg-2',
      time: 1,
      role: 'agent',
      subagent: null,
      ev: {
        t: 'text',
        text: 'hello',
      },
    },
    'happy-wire sessionEnvelopeSchema subagent=null',
  );
  expectReject(
    modules.messages.SessionProtocolMessageSchema,
    {
      role: 'session',
      content: {
        id: 'msg-3',
        time: 1,
        role: 'agent',
        ev: {
          t: 'text',
          text: 'hello',
        },
      },
      meta: null,
    },
    'happy-wire SessionProtocolMessageSchema meta=null',
  );
  expectReject(
    modules.messages.UpdateMachineBodySchema,
    {
      t: 'update-machine',
      machineId: 'machine-1',
      active: null,
    },
    'happy-wire UpdateMachineBodySchema active=null',
  );
  expectReject(
    modules.messages.UpdateMachineBodySchema,
    {
      t: 'update-machine',
      machineId: 'machine-1',
      activeAt: null,
    },
    'happy-wire UpdateMachineBodySchema activeAt=null',
  );
}

async function readFixtureFile(fileName) {
  const raw = await readFile(path.join(fixtureDir, fileName), 'utf8');
  const parsed = JSON.parse(raw);

  if (!Array.isArray(parsed)) {
    throw new Error(`${fileName} must contain an array of named fixtures`);
  }

  return parsed;
}

function validateFixtures(fixtures, schema, schemaName) {
  for (const fixture of fixtures) {
    parseFixture(schema, fixture.value, `${schemaName} fixture ${fixture.name}`);
  }
}

function parseFixture(schema, value, description) {
  const result = schema.safeParse(value);
  if (!result.success) {
    throw new Error(`${description} failed: ${formatIssues(result.error.issues)}`);
  }
}

function expectReject(schema, value, description) {
  const result = schema.safeParse(value);
  if (result.success) {
    throw new Error(`${description} unexpectedly passed`);
  }
}

function formatIssues(issues) {
  return issues
    .map((issue) => `${issue.path.length === 0 ? '<root>' : issue.path.join('.')}: ${issue.message}`)
    .join('; ');
}

function assert(value, message) {
  if (!value) {
    throw new Error(message);
  }
}

main().catch((error) => {
  console.error(error instanceof Error ? error.message : String(error));
  process.exitCode = 1;
});
