


export default async function Drive({ params }: { params: Promise<{ drive_id: string }> }){
  const drive_id = (await params).drive_id

}