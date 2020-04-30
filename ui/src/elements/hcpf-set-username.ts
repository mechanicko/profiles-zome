import { moduleConnect } from '@uprtcl/micro-orchestrator';
import { LitElement, css, html, query, property } from 'lit-element';
import { ApolloClient } from 'apollo-boost';
import { ApolloClientModule } from '@uprtcl/graphql';

import { TextField } from '@material/mwc-textfield';
import '@material/mwc-button';
import { SET_USERNAME } from '../graphql/queries';

export class SetUsername extends moduleConnect(LitElement) {
  @query('#username-field')
  usernameField!: TextField;

  @property({ type: Number })
  usernameMinLength: number = 3;

  client!: ApolloClient<any>;

  firstUpdated() {
    this.client = this.request(ApolloClientModule.bindings.Client);

    this.usernameField.validityTransform = (newValue) => {
      if (newValue.length >= this.usernameMinLength) {
        this.usernameField.setCustomValidity(
          `Username is too shot, min. ${this.usernameField} characters`
        );
        return {
          valid: false,
        };
      }

      return {
        valid: true,
      };
    };
  }

  static get styles() {
    return css`
      .row {
        display: flex;
        flex-direction: row;
      }
      .column {
        display: flex;
        flex-direction: column;
      }
    `;
  }

  setUsername() {
    this.client.mutate({
      mutation: SET_USERNAME,
      variables: {
        username: this.usernameField.value,
      },
    });
  }

  render() {
    return html`
      <div class="column">
        <mwc-textfield
          id="username-field"
          @input=${() => this.usernameField.reportValidity()}
        ></mwc-textfield>
        <mwc-button
          .disabled=${this.usernameField.validity.valid}
          label="SET USERNAME"
          @click=${() => this.setUsername()}
        ></mwc-button>
      </div>
    `;
  }
}
