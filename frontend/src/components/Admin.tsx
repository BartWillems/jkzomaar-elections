import React from "react";
import QRCode from "react-qr-code";
import ApiClient from "../Api";

interface Ballot {
  id: string;
}

const ballotUrl = (ballot: Ballot): string => {
  const url = `${process.env.REACT_APP_VOTE_URL_PREFIX}/${ballot.id}`;
  console.debug(url);
  return url;
};

// TODO: add password verification
const Admin = () => {
  const [ballot, setBallot] = React.useState<Ballot | null>(null);

  React.useEffect(() => {
    ApiClient.post("/ballots")
      .then((resp) => {
        setBallot(resp.data);
      })
      .catch((error) => {
        console.error(error);
      });
  }, []);

  return (
    <div style={{ textAlign: "center" }}>
      {ballot && <QRCode value={ballotUrl(ballot)} size={750} />}
    </div>
  );
};

export default Admin;
