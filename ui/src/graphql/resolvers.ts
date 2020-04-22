import { HolochainProvider } from '@uprtcl/holochain-provider';

import { ProfilesBindings } from '../bindings';

export const resolvers = {
  Query: {
    async allAgents(_, __, { container }) {
      const profilesProvider: HolochainProvider = container.get(
        ProfilesBindings.ProfilesProvider
      );

      return profilesProvider.call('get_all_agents', {});
    },
  },
  Mutation: {},
};
