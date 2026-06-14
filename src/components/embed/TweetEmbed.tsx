/**
 * Tweet embed component - disabled (react-tweet removed)
 */
interface TweetEmbedProps {
  id: string;
}

export default function TweetEmbed({ id }: TweetEmbedProps) {
  return (
    <div class="tweet-embed-placeholder">
      <p>Tweet embed is disabled. Tweet ID: {id}</p>
    </div>
  );
}
