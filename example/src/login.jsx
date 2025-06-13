import { useState } from "react";

export default function Login() {
  const [invitationCode, setInvitationCode] = useState("");

  return (
    <div>
      <h1>Login</h1>
      <input
        type="text"
        name="invitationCode"
        placeholder="Invitation Code"
        value={invitationCode}
        onChange={(e) => setInvitationCode(e.target.value)}
      />
      <button
        onClick={async () => {
          await handleSubmit(invitationCode);
        }}
      >
        Login
      </button>
    </div>
  );
}

async function handleSubmit(invitationCode) {
  console.log("Handle submit with invitation code", invitationCode);
  try {
    const response = await fetch(
      `https://ai.aimoverse.xyz/api/v1.0.0/auth/check-invitation-code`,
      {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ invitation_code: invitationCode }),
      }
    );
    let data = await response.json();

    console.log("Login successful with token: ", data.access_token);
    localStorage.setItem("access_token", data.access_token);
  } catch (error) {
    console.error("Login failed: ", error);
  }
}
