import { interfaces } from 'inversify';
import { GraphQlSchemaModule } from '@uprtcl/graphql';
import { MicroModule, i18nextModule } from '@uprtcl/micro-orchestrator';
import {
  HolochainConnectionModule,
  createHolochainProvider,
} from '@uprtcl/holochain-provider';

import { MyTransactions } from './elements/hcmc-my-transactions';

import en from './i18n/en.json';
import { mutualCreditTypeDefs } from './graphql/schema';
import { ProfilesBindings } from './bindings';
import { resolvers } from './graphql/resolvers';

export class ProfileModule extends MicroModule {
  static id = Symbol('holochain-profile-module');

  dependencies = [HolochainConnectionModule.id];

  static bindings = ProfilesBindings;

  constructor(protected instance: string) {
    super();
  }

  async onLoad(container: interfaces.Container) {
    const profilesProvider = createHolochainProvider(this.instance, 'profiles');

    container.bind(ProfilesBindings.ProfilesProvider).to(profilesProvider);

    customElements.define('hcmc-my-transactions', MyTransactions);
  }

  get submodules() {
    return [
      new GraphQlSchemaModule(mutualCreditTypeDefs, resolvers),
      new i18nextModule('mutual-credit', { en: en }),
    ];
  }
}
