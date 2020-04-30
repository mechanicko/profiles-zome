import { interfaces } from 'inversify';
import { GraphQlSchemaModule } from '@uprtcl/graphql';
import { MicroModule, i18nextModule } from '@uprtcl/micro-orchestrator';
import {
  HolochainConnectionModule,
  createHolochainProvider,
} from '@uprtcl/holochain-provider';

import en from './i18n/en.json';
import { ProfilesBindings } from './bindings';
import { profilesTypeDefs } from './graphql/schema';
import { resolvers } from './graphql/resolvers';
import { SetUsername } from './elements/hcpf-set-username';

export class ProfilesModule extends MicroModule {
  static id = Symbol('holochain-profile-module');

  dependencies = [HolochainConnectionModule.id];

  static bindings = ProfilesBindings;

  constructor(protected instance: string) {
    super();
  }

  async onLoad(container: interfaces.Container) {
    const profilesProvider = createHolochainProvider(this.instance, 'profiles');

    container.bind(ProfilesBindings.ProfilesProvider).to(profilesProvider);

    customElements.define('hcpf-set-username', SetUsername);
  }

  get submodules() {
    return [
      new GraphQlSchemaModule(profilesTypeDefs, resolvers),
      new i18nextModule('profiles', { en: en }),
    ];
  }
}
